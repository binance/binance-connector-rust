use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use ed25519_dalek::Signer as Ed25519Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::pkcs8::DecodePrivateKey;
use flate2::read::GzDecoder;
use hex;
use hmac::{Hmac, Mac};
use http::HeaderMap;
use http::HeaderValue;
use http::header::ACCEPT_ENCODING;
use once_cell::sync::OnceCell;
use openssl::{hash::MessageDigest, pkey::PKey, sign::Signer as OpenSslSigner};
use rand::RngCore;
use regex::Captures;
use regex::Regex;
use reqwest::Client;
use reqwest::Proxy;
use reqwest::{Method, Request};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use sha2::Sha256;
use std::fmt::Display;
use std::hash::BuildHasher;
use std::sync::LazyLock;
use std::{
    collections::BTreeMap,
    collections::HashMap,
    fs,
    io::Read,
    path::Path,
    time::Duration,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::sleep;
use tracing::info;
use url::{Url, form_urlencoded::Serializer};

use super::config::{
    ConfigurationRestApi, ConfigurationWebsocketApi, HttpAgent, PrivateKey, ProxyConfig,
};
use super::errors::ConnectorError;
use super::models::{Interval, RateLimitType, RestApiRateLimit, RestApiResponse, TimeUnit};
use super::websocket::WebsocketMessageSendOptions;

pub(crate) static ID_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9a-f]{32}$").unwrap());
static PLACEHOLDER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(@)?<([^>]+)>").unwrap());

/// A generator for creating cryptographic signatures with support for various key types and configurations.
///
/// This struct manages different authentication mechanisms including API secrets, private keys,
/// and supports multiple key formats (file-based or raw bytes). It uses lazy initialization
/// for key loading and supports different cryptographic key types like OpenSSL private keys
/// and Ed25519 signing keys.
///
/// # Fields
/// * `api_secret`: Optional API secret for signature generation
/// * `private_key`: Optional private key source (file or raw bytes)
/// * `private_key_passphrase`: Optional passphrase for decrypting private keys
/// * `raw_key_data`: Lazily initialized raw key data as a string
/// * `key_object`: Lazily initialized OpenSSL private key
/// * `ed25519_signing_key`: Lazily initialized Ed25519 signing key
#[derive(Debug, Default, Clone)]
pub struct SignatureGenerator {
    api_secret: Option<String>,
    private_key: Option<PrivateKey>,
    private_key_passphrase: Option<String>,
    raw_key_data: OnceCell<String>,
    key_object: OnceCell<PKey<openssl::pkey::Private>>,
    ed25519_signing_key: OnceCell<SigningKey>,
}

impl SignatureGenerator {
    #[must_use]
    pub fn new(
        api_secret: Option<String>,
        private_key: Option<PrivateKey>,
        private_key_passphrase: Option<String>,
    ) -> Self {
        SignatureGenerator {
            api_secret,
            private_key,
            private_key_passphrase,
            raw_key_data: OnceCell::new(),
            key_object: OnceCell::new(),
            ed25519_signing_key: OnceCell::new(),
        }
    }

    /// Retrieves the raw key data from a private key source.
    ///
    /// This method lazily initializes the raw key data by reading it from either a file path
    /// or a raw byte array. If the key is from a file, it checks for file existence before reading.
    /// If the key is provided as raw bytes, it converts them to a UTF-8 string.
    ///
    /// # Returns
    /// A reference to the raw key data as a `String`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - No private key is provided
    /// - The private key file does not exist
    /// - The private key file cannot be read
    fn get_raw_key_data(&self) -> Result<&String> {
        self.raw_key_data.get_or_try_init(|| {
            let pk = self
                .private_key
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No private_key provided"))?;
            match pk {
                PrivateKey::File(path) => {
                    if Path::new(path).exists() {
                        fs::read_to_string(path)
                            .with_context(|| format!("Failed to read private key file: {path}"))
                    } else {
                        Err(anyhow::anyhow!("Private key file does not exist: {}", path))
                    }
                }
                PrivateKey::Raw(bytes) => Ok(String::from_utf8_lossy(bytes).to_string()),
            }
        })
    }

    /// Retrieves the private key object, lazily initializing it from raw key data.
    ///
    /// This method attempts to parse the private key from PEM format, supporting both
    /// passphrase-protected and unprotected keys. It uses the raw key data obtained
    /// from `get_raw_key_data()` and attempts to create an OpenSSL private key object.
    ///
    /// # Returns
    /// A reference to the parsed private key as a `PKey<openssl::pkey::Private>`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The key cannot be parsed from PEM format
    /// - A passphrase is required but incorrect
    /// - The key data is invalid
    fn get_key_object(&self) -> Result<&PKey<openssl::pkey::Private>> {
        self.key_object.get_or_try_init(|| {
            let key_data = self.get_raw_key_data()?;
            if let Some(pass) = self.private_key_passphrase.as_ref() {
                PKey::private_key_from_pem_passphrase(key_data.as_bytes(), pass.as_bytes())
                    .context("Failed to parse private key with passphrase")
            } else {
                PKey::private_key_from_pem(key_data.as_bytes())
                    .context("Failed to parse private key")
            }
        })
    }

    /// Retrieves the Ed25519 signing key, lazily initializing it from raw key data.
    ///
    /// This method attempts to parse an Ed25519 private key from a PEM-formatted input,
    /// extracting the base64-encoded key material and converting it to a `SigningKey`.
    ///
    /// # Returns
    /// A reference to the parsed Ed25519 signing key.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The key cannot be base64 decoded
    /// - The key cannot be parsed from PKCS8 DER format
    fn get_ed25519_signing_key(
        &self,
        key_obj: &PKey<openssl::pkey::Private>,
    ) -> Result<&SigningKey> {
        self.ed25519_signing_key.get_or_try_init(|| {
            let der = key_obj
                .private_key_to_der()
                .context("Failed to export Ed25519 key to DER")?;
            SigningKey::from_pkcs8_der(&der)
                .map_err(|e| anyhow::anyhow!("Failed to parse Ed25519 key: {}", e))
        })
    }

    /// Generates a signature for the given query parameters using either HMAC-SHA256 or asymmetric key signing.
    ///
    /// # Arguments
    ///
    /// * `query_params` - A map of query parameters to be signed
    ///
    /// # Returns
    ///
    /// A base64-encoded signature string
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No API secret or private key is provided
    /// - Key initialization fails
    /// - Signing process encounters an error
    /// - An unsupported key type is used
    ///
    /// # Supported Key Types
    /// - HMAC with API secret
    /// - RSA private key
    /// - ED25519 private key
    pub fn get_signature(&self, query_params: &BTreeMap<String, Value>) -> Result<String> {
        let params = build_query_string(query_params)?;

        if let Some(secret) = self.api_secret.as_ref() {
            if self.private_key.is_none() {
                let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
                    .context("HMAC key initialization failed")?;
                mac.update(params.as_bytes());
                let result = mac.finalize().into_bytes();
                return Ok(hex::encode(result));
            }
        }

        if self.private_key.is_some() {
            let key_obj = self.get_key_object()?;
            match key_obj.id() {
                openssl::pkey::Id::RSA => {
                    let mut signer = OpenSslSigner::new(MessageDigest::sha256(), key_obj)
                        .context("Failed to create RSA signer")?;
                    signer
                        .update(params.as_bytes())
                        .context("Failed to update RSA signer")?;
                    let sig = signer.sign_to_vec().context("RSA signing failed")?;
                    return Ok(general_purpose::STANDARD.encode(sig));
                }
                openssl::pkey::Id::ED25519 => {
                    let signing_key = self.get_ed25519_signing_key(key_obj)?;
                    let signature = signing_key.sign(params.as_bytes());
                    return Ok(general_purpose::STANDARD.encode(signature.to_bytes()));
                }
                other => {
                    return Err(anyhow::anyhow!(
                        "Unsupported private key type: {:?}. Must be RSA or ED25519.",
                        other
                    ));
                }
            }
        }

        Err(anyhow::anyhow!(
            "Either 'api_secret' or 'private_key' must be provided for signed requests."
        ))
    }
}

/// Builds a reqwest HTTP client with configurable timeout, keep-alive, proxy, and custom agent settings.
///
/// # Arguments
///
/// * `timeout` - Timeout duration in milliseconds for HTTP requests
/// * `keep_alive` - Whether to enable HTTP keep-alive connections
/// * `proxy` - Optional proxy configuration for routing requests
/// * `agent` - Optional custom HTTP agent configuration function
///
/// # Returns
///
/// A configured `reqwest::Client` instance
///
/// # Panics
///
/// Panics if the client cannot be built with the provided configuration
///
/// # Examples
///
///
/// let client = `build_client(5000`, true, None, None);
///
#[must_use]
pub fn build_client(
    timeout: u64,
    keep_alive: bool,
    proxy: Option<&ProxyConfig>,
    agent: Option<HttpAgent>,
) -> Client {
    let builder = Client::builder().timeout(Duration::from_millis(timeout));

    let mut builder = if keep_alive {
        builder
    } else {
        builder.pool_idle_timeout(Some(Duration::from_secs(0)))
    };

    if let Some(proxy_conf) = proxy {
        let protocol = proxy_conf
            .protocol
            .clone()
            .unwrap_or_else(|| "http".to_string());
        let proxy_url = format!("{}://{}:{}", protocol, proxy_conf.host, proxy_conf.port);
        let mut proxy_builder = Proxy::all(&proxy_url).expect("Failed to create proxy from URL");
        if let Some(auth) = &proxy_conf.auth {
            proxy_builder = proxy_builder.basic_auth(&auth.username, &auth.password);
        }
        builder = builder.proxy(proxy_builder);
    }

    if let Some(HttpAgent(agent_fn)) = agent {
        builder = (agent_fn)(builder);
    }

    info!("Client builder {:?}", builder);

    builder.build().expect("Failed to build reqwest client")
}

/// Generates a user agent string for the current module.
///
/// # Arguments
///
/// * `product` - A string slice representing the product.
///
/// # Returns
///
/// A formatted user agent string containing:
/// - Package name
/// - Product
/// - Package version
/// - Rust compiler version
/// - Operating system
/// - Architecture
///
/// # Examples
///
///
/// let `user_agent` = `build_user_agent("spot`");
/// // Might return something like: "`binance_sdk/spot/1.0.0` (Rust/rustc 1.87.0; linux; `x86_64`;)"
///
#[must_use]
pub fn build_user_agent(product: &str) -> String {
    format!(
        "{}/{}/{} (Rust/{}; {}; {})",
        env!("CARGO_PKG_NAME"),
        product,
        env!("CARGO_PKG_VERSION"),
        env!("RUSTC_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}

/// Validates the time unit string and returns an optional normalized time unit.
///
/// # Arguments
///
/// * `time_unit` - A string representing the time unit to validate.
///
/// # Returns
///
/// * `Ok(None)` if an empty string is provided
/// * `Ok(Some(time_unit))` if the time unit is 'MILLISECOND', 'MICROSECOND', 'millisecond', or 'microsecond'
/// * `Err` with an error message if an invalid time unit is provided
///
/// # Errors
///
/// Returns `Err(anyhow::Error)` if `time_unit` is non-empty and not one of the allowed values.
///
/// # Examples
///
/// let result = `validate_time_unit("MILLISECOND`");
/// `assert!(result.is_ok())`;
///
/// let result = `validate_time_unit`("");
/// `assert!(result.is_ok()` && `result.unwrap().is_none()`);
///
/// let result = `validate_time_unit("SECOND`");
/// `assert!(result.is_err())`;
///
pub fn validate_time_unit(time_unit: &str) -> Result<Option<&str>, anyhow::Error> {
    match time_unit {
        "" => Ok(None),
        "MILLISECOND" | "MICROSECOND" | "millisecond" | "microsecond" => Ok(Some(time_unit)),
        _ => Err(anyhow::anyhow!(
            "time_unit must be either 'MILLISECOND' or 'MICROSECOND'"
        )),
    }
}

/// Returns the current timestamp in milliseconds since the Unix epoch.
///
/// # Returns
///
/// * A `u128` representing the current timestamp in milliseconds.
///
/// # Panics
///
/// Panics if the system time is set to a time before the Unix epoch.
///
/// # Examples
///
///
/// let timestamp = `get_timestamp()`;
/// println!("Current timestamp: {}", timestamp);
///
#[must_use]
pub fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

/// Asynchronously pauses the current task for a specified number of milliseconds.
///
/// # Arguments
///
/// * `ms` - The number of milliseconds to pause the task.
///
/// # Examples
///
///
/// let _ = delay(100).await; // Pause for 100 milliseconds
///
pub async fn delay(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

/// Builds a query string from a map of key-value parameters.
///
/// Converts various JSON `Value` types into URL query string segments, handling:
/// - Strings, booleans, and numbers as direct key-value pairs
/// - Arrays of strings, booleans, or numbers as comma-separated values
/// - Nested arrays serialized as JSON strings
///
/// # Arguments
///
/// * `params` - A map of parameter names to their corresponding JSON values
///
/// # Returns
///
/// * `Result<String, anyhow::Error>` - A query string with URL-encoded parameters, or an error
///
/// # Errors
///
/// Returns an error if an object value is encountered or JSON serialization fails
pub fn build_query_string(params: &BTreeMap<String, Value>) -> Result<String, anyhow::Error> {
    let mut segments = Vec::with_capacity(params.len());

    for (key, value) in params {
        if value.is_null() {
            continue;
        }

        let value_str = match value {
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Array(_) | Value::Object(_) => serde_json::to_string(value)
                .with_context(|| format!("failed to JSON-serialize `{}`", key))?,
            Value::Null => unreachable!(),
        };

        let mut ser = Serializer::new(String::new());
        ser.append_pair(key, &value_str);
        segments.push(ser.finish());
    }

    Ok(segments.join("&"))
}

/// Determines whether a request should be retried based on:
/// - HTTP method (only GET or DELETE are retriable)
/// - HTTP status (500, 502, 503, 504)
/// - Number of retries left.
///
/// `error` is the reqwest error, `method` is the HTTP method (e.g. "GET"),
/// and `retries_left` is the number of remaining retries.
#[must_use]
pub fn should_retry_request(
    error: &reqwest::Error,
    method: Option<&str>,
    retries_left: Option<usize>,
) -> bool {
    let method = method.unwrap_or("");
    let is_retriable_method =
        method.eq_ignore_ascii_case("GET") || method.eq_ignore_ascii_case("DELETE");

    let status = error.status().map_or(0, |s| s.as_u16());
    let is_retriable_status = [500, 502, 503, 504].contains(&status);

    let retries_left = retries_left.unwrap_or(0);
    retries_left > 0 && is_retriable_method && (is_retriable_status || error.status().is_none())
}

/// Parses rate limit headers from a `HashMap` of headers and returns a vector of `RestApiRateLimit`.
///
/// This function extracts rate limit information from headers with specific patterns (x-mbx-used-weight or x-mbx-order-count)
/// and converts them into `RestApiRateLimit` structures. It handles different intervals (seconds, minutes, hours, days)
/// and distinguishes between request weight and order rate limits.
///
/// # Arguments
///
/// * `headers` - A reference to a `HashMap` containing HTTP headers
///
/// # Returns
///
/// A `Vec<RestApiRateLimit>` containing parsed rate limit information
///
/// # Panics
///
/// * If the static regex fails to compile (via `Regex::new(...).unwrap()`), which can only happen if the literal pattern is invalid.  
/// * If a matching header’s key doesn’t actually contain both capture groups (so `caps.get(2).unwrap()` or `caps.get(3).unwrap()` fails).
///
/// # Examples
///
/// let headers: `HashMap`<String, String> = // ... headers with rate limit information
/// let `rate_limits` = `parse_rate_limit_headers(&headers)`;
///
#[must_use]
pub fn parse_rate_limit_headers<S>(headers: &HashMap<String, String, S>) -> Vec<RestApiRateLimit>
where
    S: BuildHasher,
{
    let mut rate_limits = Vec::new();
    let re = Regex::new(r"x-mbx-(used-weight|order-count)-(\d+)([smhd])").unwrap();
    for (key, value) in headers {
        let normalized_key = key.to_lowercase();
        if normalized_key.starts_with("x-mbx-used-weight-")
            || normalized_key.starts_with("x-mbx-order-count-")
        {
            if let Some(caps) = re.captures(&normalized_key) {
                let interval_num: u32 = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
                let interval_letter = caps.get(3).unwrap().as_str().to_uppercase();
                let interval = match interval_letter.as_str() {
                    "S" => Interval::Second,
                    "M" => Interval::Minute,
                    "H" => Interval::Hour,
                    "D" => Interval::Day,
                    _ => continue,
                };
                let count: u32 = value.parse().unwrap_or(0);
                let rate_limit_type = if normalized_key.starts_with("x-mbx-used-weight-") {
                    RateLimitType::RequestWeight
                } else {
                    RateLimitType::Orders
                };
                rate_limits.push(RestApiRateLimit {
                    rate_limit_type,
                    interval,
                    interval_num,
                    count,
                    retry_after: headers.get("retry-after").and_then(|v| v.parse().ok()),
                });
            }
        }
    }
    rate_limits
}

/// Sends an HTTP request with retry and error handling capabilities.
///
/// # Parameters
///
/// - `req`: The HTTP request to be sent
/// - `configuration`: REST API configuration containing client, retry settings, and other parameters
///
/// # Returns
///
/// A `Result` containing a `RestApiResponse` with deserialized data, or a `ConnectorError` if the request fails
///
/// # Errors
///
/// Returns various `ConnectorError` types based on HTTP response status, such as:
/// - `BadRequestError`
/// - `UnauthorizedError`
/// - `ForbiddenError`
/// - `NotFoundError`
/// - `RateLimitBanError`
/// - `TooManyRequestsError`
/// - `ServerError`
/// - `ConnectorClientError`
///
/// # Behavior
///
/// - Supports request retries with configurable backoff
/// - Handles gzip-encoded responses
/// - Parses rate limit headers
/// - Provides detailed error handling for different HTTP status codes
pub async fn http_request<T: DeserializeOwned + Send + 'static>(
    req: Request,
    configuration: &ConfigurationRestApi,
) -> Result<RestApiResponse<T>, ConnectorError> {
    let client = &configuration.client;
    let retries = configuration.retries as usize;
    let backoff = configuration.backoff;
    let mut attempt = 0;

    loop {
        let req_clone = req
            .try_clone()
            .context("Failed to clone request")
            .map_err(|e| ConnectorError::ConnectorClientError(e.to_string()))?;
        match client.execute(req_clone).await {
            Ok(response) => {
                let status = response.status();
                let headers_map: HashMap<String, String> = response
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect();

                let raw_bytes = match response.bytes().await {
                    Ok(b) => b,
                    Err(e) => {
                        attempt += 1;
                        if attempt <= retries {
                            continue;
                        }
                        return Err(ConnectorError::ConnectorClientError(format!(
                            "Failed to get response bytes: {e}"
                        )));
                    }
                };

                let content = if headers_map
                    .get("content-encoding")
                    .is_some_and(|enc| enc.to_lowercase().contains("gzip"))
                {
                    let mut decoder = GzDecoder::new(&raw_bytes[..]);
                    let mut decompressed = String::new();
                    decoder
                        .read_to_string(&mut decompressed)
                        .context("Failed to decompress gzip response")
                        .map_err(|e| ConnectorError::ConnectorClientError(e.to_string()))?;
                    decompressed
                } else {
                    String::from_utf8(raw_bytes.to_vec())
                        .context("Failed to convert response to UTF-8")
                        .map_err(|e| ConnectorError::ConnectorClientError(e.to_string()))?
                };

                let rate_limits = parse_rate_limit_headers(&headers_map);

                if status.is_client_error() || status.is_server_error() {
                    let error_msg = serde_json::from_str::<serde_json::Value>(&content)
                        .ok()
                        .and_then(|v| {
                            v.get("msg")
                                .and_then(|m| m.as_str())
                                .map(std::string::ToString::to_string)
                        })
                        .unwrap_or_else(|| content.clone());

                    match status.as_u16() {
                        400 => return Err(ConnectorError::BadRequestError(error_msg)),
                        401 => return Err(ConnectorError::UnauthorizedError(error_msg)),
                        403 => return Err(ConnectorError::ForbiddenError(error_msg)),
                        404 => return Err(ConnectorError::NotFoundError(error_msg)),
                        418 => return Err(ConnectorError::RateLimitBanError(error_msg)),
                        429 => return Err(ConnectorError::TooManyRequestsError(error_msg)),
                        s if (500..600).contains(&s) => {
                            return Err(ConnectorError::ServerError {
                                msg: format!("Server error: {s}"),
                                status_code: Some(s),
                            });
                        }
                        _ => return Err(ConnectorError::ConnectorClientError(error_msg)),
                    }
                }

                let raw = content.clone();
                return Ok(RestApiResponse {
                    data_fn: Box::new(move || {
                        Box::pin(async move {
                            let parsed: T = serde_json::from_str(&raw)
                                .map_err(|e| ConnectorError::ConnectorClientError(e.to_string()))?;
                            Ok(parsed)
                        })
                    }),
                    status: status.as_u16(),
                    headers: headers_map,
                    rate_limits: if rate_limits.is_empty() {
                        None
                    } else {
                        Some(rate_limits)
                    },
                });
            }
            Err(e) => {
                attempt += 1;
                if should_retry_request(&e, Some(req.method().as_str()), Some(retries - attempt)) {
                    delay(backoff * attempt as u64).await;
                    continue;
                }
                return Err(ConnectorError::ConnectorClientError(format!(
                    "HTTP request failed: {e}"
                )));
            }
        }
    }
}

/// Sends an HTTP request to a REST API endpoint with optional authentication and configuration.
///
/// # Parameters
///
/// - `configuration`: REST API configuration containing client, base path, and authentication details
/// - `endpoint`: The specific API endpoint path to send the request to
/// - `method`: HTTP method for the request (GET, POST, etc.)
/// - `params`: Parameters to be sent with the request, as a key-value map
/// - `time_unit`: Optional time unit for the request header
/// - `is_signed`: Optional flag to indicate whether the request requires authentication
///
/// # Returns
///
/// A `RestApiResponse` containing the deserialized response data, or an error if the request fails
///
/// # Panics
///
/// This function will panic if any of the following `.unwrap()` calls fail:
/// - Parsing the literal `"application/json"` into a header value (should never fail)  
/// - Parsing `configuration.user_agent` or `configuration.api_key` into header values  
/// - Parsing the literal `"gzip, deflate, br"` into a header value when `compression` is enabled  
///
/// # Errors
///
/// Returns an `anyhow::Result` which can contain various connector-related errors during request processing
pub async fn send_request<T: DeserializeOwned + Send + 'static>(
    configuration: &ConfigurationRestApi,
    endpoint: &str,
    method: Method,
    mut params: BTreeMap<String, Value>,
    time_unit: Option<TimeUnit>,
    is_signed: bool,
) -> anyhow::Result<RestApiResponse<T>> {
    let base = configuration.base_path.as_deref().unwrap_or("");
    let full_url = reqwest::Url::parse(base)
        .and_then(|u| u.join(endpoint))
        .context("Failed to join base URL and endpoint")?
        .to_string();

    let signature = is_signed
        .then(|| {
            let timestamp = get_timestamp();
            params.insert("timestamp".to_string(), json!(timestamp));
            configuration.signature_gen.get_signature(&params)
        })
        .transpose()?;

    let mut url = Url::parse(&full_url)?;
    {
        let mut pairs = url.query_pairs_mut();
        for (key, value) in &params {
            let val_str = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            pairs.append_pair(key, &val_str);
        }
        if let Some(signature) = signature {
            pairs.append_pair("signature", &signature);
        }
    }

    let mut headers = HeaderMap::new();

    let forbidden = ["host", "authorization", "cookie", ":method", ":path"]
        .into_iter()
        .map(str::to_ascii_lowercase)
        .collect::<std::collections::HashSet<_>>();

    if let Some(custom) = &configuration.custom_headers {
        for (raw_name, raw_val) in custom {
            let name = raw_name.trim();
            if forbidden.contains(&name.to_ascii_lowercase()) {
                continue;
            }
            if let (Ok(header_name), Ok(header_val)) = (
                name.parse::<reqwest::header::HeaderName>(),
                HeaderValue::from_str(raw_val),
            ) {
                headers.append(header_name, header_val);
            }
        }
    }

    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", configuration.user_agent.parse().unwrap());
    if let Some(api_key) = &configuration.api_key {
        headers.insert("X-MBX-APIKEY", HeaderValue::from_str(api_key)?);
    }

    if configuration.compression {
        headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    }

    let time_unit_to_apply = time_unit.or(configuration.time_unit);
    if let Some(time_unit) = time_unit_to_apply {
        headers.insert("X-MBX-TIME-UNIT", time_unit.as_upper_str().parse()?);
    }

    let req_builder = configuration.client.request(method, url).headers(headers);
    let req = req_builder.build()?;

    Ok(http_request::<T>(req, configuration).await?)
}

/// Generates a random hexadecimal string of 32 characters.
///
/// Uses the thread-local random number generator to fill a 16-byte buffer,
/// which is then encoded into a hexadecimal string.
///
/// # Returns
///
/// A randomly generated 32-character hexadecimal string.
#[must_use]
pub fn random_string() -> String {
    let mut buf = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut buf);
    hex::encode(buf)
}

/// Removes entries with empty or null values from an iterator of key-value pairs.
///
/// # Arguments
///
/// * `entries` - An iterator of key-value pairs where keys are strings and values are of type `Value`.
///
/// # Returns
///
/// A `BTreeMap` containing only the key-value pairs where the value is neither `null` nor an empty string.
///
/// # Examples
///
///
/// let entries = vec![
///     ("`key1".to_string()`, `Value::String("value1".to_string())`),
///     ("`key2".to_string()`, `Value::Null`),
///     ("`key3".to_string()`, `Value::String("".to_string())`),
/// ];
/// let filtered = `remove_empty_value(entries)`;
/// // filtered will only contain the first key-value pair
///
pub fn remove_empty_value<I>(entries: I) -> BTreeMap<String, Value>
where
    I: IntoIterator<Item = (String, Value)>,
{
    entries
        .into_iter()
        .filter(|(_, value)| match value {
            Value::Null => false,
            Value::String(s) if s.is_empty() => false,
            _ => true,
        })
        .collect()
}

/// Creates a sorted copy of a `BTreeMap` of parameters.
///
/// # Arguments
///
/// * `params` - A reference to a `BTreeMap` containing string keys and Value values.
///
/// # Returns
///
/// A new `BTreeMap` with the same key-value pairs as the input, sorted by keys.
///
/// # Examples
///
///
/// let params = `BTreeMap::from`([
///     ("`z".to_string()`, `Value::String("value1".to_string())`),
///     ("`a".to_string()`, `Value::String("value2".to_string())`),
/// ]);
/// let `sorted_params` = `sort_object_params(&params)`;
/// // `sorted_params` will have keys sorted in ascending order
///
#[must_use]
pub fn sort_object_params(params: &BTreeMap<String, Value>) -> BTreeMap<String, Value> {
    let mut sorted = BTreeMap::new();
    for (k, v) in params {
        sorted.insert(k.clone(), v.clone());
    }
    sorted
}

/// Normalizes a WebSocket streams key by converting it to lowercase and removing underscores and hyphens.
///
/// # Arguments
///
/// * `key` - The input key to be normalized
///
/// # Returns
///
/// A normalized string with lowercase characters and no underscores or hyphens
fn normalize_ws_streams_key(key: &str) -> String {
    key.to_lowercase().replace(&['_', '-'][..], "")
}

/// Replaces placeholders in a WebSocket stream key with corresponding values from a variables map.
///
/// # Arguments
///
/// * `input` - The input string containing placeholders to be replaced
/// * `variables` - A `HashMap` of key-value pairs used for placeholder substitution
///
/// # Returns
///
/// A modified string with placeholders replaced by their corresponding values,
/// with special handling for normalization, lowercasing, and '@' symbol stripping.
///
/// # Panics
///
/// Panics if the input string contains an invalid placeholder format.
///
/// # Examples
///
///
/// let input = "/<symbol>@ticker";
/// let variables = `HashMap::from`([("symbol", "BTCUSDT")]);
/// let result = `replace_websocket_streams_placeholders(input`, &variables);
/// // Possible result: "btcusdt@ticker"
///
pub fn replace_websocket_streams_placeholders<V, S>(
    input: &str,
    variables: &HashMap<&str, V, S>,
) -> String
where
    V: Display,
    S: BuildHasher,
{
    let original = input;

    // Drop a leading slash for processing
    let body = original.strip_prefix('/').unwrap_or(original);

    // Normalize variables into String→String map
    let normalized: HashMap<String, String> = variables
        .iter()
        .map(|(k, v)| (normalize_ws_streams_key(k), v.to_string()))
        .collect();

    // Replace all placeholders, preserving any '@' prefix captured by the regex
    let replaced = PLACEHOLDER_RE
        .replace_all(body, |caps: &Captures| {
            let prefix = caps.get(1).map_or("", |m| m.as_str());
            let key = normalize_ws_streams_key(caps.get(2).unwrap().as_str());
            let val = normalized.get(&key).cloned().unwrap_or_default();
            format!("{prefix}{val}")
        })
        .into_owned();

    // Strip any trailing '@'
    let stripped = replaced.trim_end_matches('@').to_string();

    // Only lowercase head if original started with '/' and first placeholder at start
    // (cases where `symbol` or `pair` are used and they are not lower-cased)
    let should_lower_head =
        original.starts_with('/') && PLACEHOLDER_RE.find(body).is_some_and(|m| m.start() == 0);

    // Lowercase only that first placeholder's value

    if should_lower_head {
        if let Some(caps) = PLACEHOLDER_RE.captures(body) {
            let key = normalize_ws_streams_key(caps.get(2).unwrap().as_str());
            let first_val = normalized.get(&key).cloned().unwrap_or_default();
            if stripped.starts_with(&first_val) {
                let tail = &stripped[first_val.len()..];
                format!("{}{}", first_val.to_lowercase(), tail)
            } else {
                stripped.clone()
            }
        } else {
            stripped.clone()
        }
    } else {
        stripped.clone()
    }
}

/// Builds a WebSocket API message with optional authentication and signature generation.
///
/// # Arguments
///
/// * `configuration` - Configuration for the WebSocket API
/// * `method` - The API method to be called
/// * `payload` - A map of parameters for the API request
/// * `options` - Options for sending the WebSocket message
/// * `skip_auth` - Flag to skip authentication if true
///
/// # Returns
///
/// A tuple containing the message ID and the constructed JSON request
///
/// # Panics
///
/// Panics if an API key is required but not set, or if signature generation fails
pub fn build_websocket_api_message(
    configuration: &ConfigurationWebsocketApi,
    method: &str,
    mut payload: BTreeMap<String, Value>,
    options: &WebsocketMessageSendOptions,
    skip_auth: bool,
) -> (String, serde_json::Value) {
    let id = payload
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| ID_REGEX.is_match(s))
        .map_or_else(random_string, String::from);

    payload.remove("id");

    let mut params = remove_empty_value(payload);

    if (options.with_api_key || options.is_signed) && !skip_auth {
        params.insert(
            "apiKey".into(),
            Value::String(configuration.api_key.clone().expect("API key must be set")),
        );
    }

    if options.is_signed {
        let ts = get_timestamp();
        let ts_i64 = i64::try_from(ts).expect("timestamp fits in i64");
        params.insert("timestamp".into(), Value::Number(ts_i64.into()));

        let mut sorted = sort_object_params(&params);
        if !skip_auth {
            let sig = configuration
                .signature_gen
                .get_signature(&sorted)
                .expect("signature generation");
            sorted.insert("signature".into(), Value::String(sig));
        }
        params = sorted.into_iter().collect();
    }

    let request = json!({
        "id": id,
        "method": method,
        "params": params,
    });

    (id, request)
}

#[cfg(test)]
mod tests {
    use crate::TOKIO_SHARED_RT;

    mod build_client {
        use std::{
            sync::{Arc, Mutex},
            time::{Duration, Instant},
        };

        use reqwest::ClientBuilder;

        use crate::{
            common::utils::build_client,
            config::{HttpAgent, ProxyAuth, ProxyConfig},
        };

        use super::TOKIO_SHARED_RT;

        #[test]
        fn enforces_timeout() {
            TOKIO_SHARED_RT.block_on(async {
                let client = build_client(100, true, None, None);
                let start = Instant::now();
                let res = client.get("http://10.255.255.1").send().await;
                assert!(
                    res.is_err(),
                    "expected an error (timeout or connect) but got {res:?}"
                );
                let elapsed = start.elapsed();
                assert!(
                    elapsed < Duration::from_millis(500),
                    "timed out too slowly: {elapsed:?}"
                );
            });
        }

        #[test]
        fn builds_with_keep_alive_disabled() {
            let client = build_client(200, false, None, None);
            let _: reqwest::Client = client;
        }

        #[test]
        #[should_panic(expected = "Failed to create proxy from URL")]
        fn invalid_proxy_url_panics() {
            let bad_proxy = ProxyConfig {
                protocol: Some("http".to_string()),
                host: String::new(),
                port: 8080,
                auth: None,
            };
            let _ = build_client(1_000, true, Some(&bad_proxy), None);
        }

        #[test]
        fn builds_with_proxy_and_auth() {
            let proxy = ProxyConfig {
                protocol: Some("https".to_string()),
                host: "127.0.0.1".to_string(),
                port: 3128,
                auth: Some(ProxyAuth {
                    username: "alice".to_string(),
                    password: "secret".to_string(),
                }),
            };
            let client = build_client(2_000, true, Some(&proxy), None);
            let _: reqwest::Client = client;
        }

        #[test]
        fn custom_agent_invoked() {
            let called = Arc::new(Mutex::new(false));
            let called_clone = Arc::clone(&called);

            let agent = HttpAgent(Arc::new(move |builder: ClientBuilder| {
                *called_clone.lock().unwrap() = true;
                builder
            }));

            let client = build_client(1_000, true, None, Some(agent));
            assert!(*called.lock().unwrap(), "agent closure wasn’t invoked");
            let _: reqwest::Client = client;
        }
    }

    mod build_user_agent {
        use crate::common::utils::build_user_agent;

        #[test]
        fn build_user_agent_contains_crate_product_and_rust_info() {
            let product = "product";
            let user_agent = build_user_agent(product);

            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            let rustc = env!("RUSTC_VERSION");
            let os = std::env::consts::OS;
            let arch = std::env::consts::ARCH;

            let expected_prefix = format!("{name}/{product}/{version} (Rust/");
            assert!(
                user_agent.starts_with(&expected_prefix),
                "prefix mismatch: {user_agent}"
            );

            assert!(
                user_agent.contains(rustc),
                "user agent missing RUSTC_VERSION: {user_agent}"
            );

            assert!(
                user_agent.contains(&format!("; {os}")),
                "user agent missing OS: {user_agent}"
            );
            assert!(
                user_agent.contains(&format!("; {arch}")),
                "user agent missing ARCH: {user_agent}"
            );
        }

        #[test]
        fn build_user_agent_is_deterministic() {
            let product = "product";
            let user_agent1 = build_user_agent(product);
            let user_agent2 = build_user_agent(product);
            assert_eq!(
                user_agent1, user_agent2,
                "user agent should be the same on repeated calls"
            );
        }
    }

    mod validate_time_unit {
        use crate::common::utils::validate_time_unit;

        #[test]
        fn empty_string_returns_none() {
            let res = validate_time_unit("").expect("Should not error on empty string");
            assert_eq!(res, None);
        }

        #[test]
        fn uppercase_millisecond() {
            let res = validate_time_unit("MILLISECOND").expect("Should accept MILLISECOND");
            assert_eq!(res, Some("MILLISECOND"));
        }

        #[test]
        fn uppercase_microsecond() {
            let res = validate_time_unit("MICROSECOND").expect("Should accept MICROSECOND");
            assert_eq!(res, Some("MICROSECOND"));
        }

        #[test]
        fn lowercase_millisecond() {
            let res = validate_time_unit("millisecond").expect("Should accept millisecond");
            assert_eq!(res, Some("millisecond"));
        }

        #[test]
        fn lowercase_microsecond() {
            let res = validate_time_unit("microsecond").expect("Should accept microsecond");
            assert_eq!(res, Some("microsecond"));
        }

        #[test]
        fn invalid_value_returns_err() {
            let err = validate_time_unit("SECOND").unwrap_err();
            let msg = format!("{err}");
            assert!(msg.contains("time_unit must be either 'MILLISECOND' or 'MICROSECOND'"));
        }

        #[test]
        fn partial_match_returns_err() {
            let err = validate_time_unit("MILLI").unwrap_err();
            let msg = format!("{err}");
            assert!(msg.contains("time_unit must be either 'MILLISECOND' or 'MICROSECOND'"));
        }
    }

    mod get_timestamp {
        use crate::common::utils::get_timestamp;
        use std::{
            thread::sleep,
            time::{Duration, SystemTime, UNIX_EPOCH},
        };

        #[test]
        fn timestamp_is_within_system_time_bounds() {
            let before = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX_EPOCH")
                .as_millis();
            let ts = get_timestamp();
            let after = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX_EPOCH")
                .as_millis();

            assert!(
                ts >= before,
                "timestamp {ts} is before captured before time {before}"
            );
            assert!(
                ts <= after,
                "timestamp {ts} is after captured after time {after}"
            );
        }

        #[test]
        fn timestamps_are_monotonic() {
            let t1 = get_timestamp();
            sleep(Duration::from_millis(1));
            let t2 = get_timestamp();
            assert!(
                t2 >= t1,
                "second timestamp {t2} is not >= first timestamp {t1}"
            );
        }
    }

    mod build_query_string {
        use std::collections::BTreeMap;

        use anyhow::Result;
        use serde_json::{Value, json};
        use url::form_urlencoded::Serializer;

        use crate::common::utils::build_query_string;

        fn mk_map(pairs: Vec<(&str, Value)>) -> BTreeMap<String, Value> {
            let mut m = BTreeMap::new();
            for (k, v) in pairs {
                m.insert(k.to_string(), v);
            }
            m
        }

        #[test]
        fn empty_map_returns_empty_string() -> Result<()> {
            let params = BTreeMap::new();
            let qs = build_query_string(&params)?;
            assert_eq!(qs, "");
            Ok(())
        }

        #[test]
        fn string_and_number_and_bool() -> Result<()> {
            let params = mk_map(vec![
                ("foo", json!("bar")),
                ("num", json!(42)),
                ("flag", json!(true)),
            ]);
            let qs = build_query_string(&params)?;
            assert_eq!(qs, "flag=true&foo=bar&num=42");
            Ok(())
        }

        #[test]
        fn null_is_skipped() -> Result<()> {
            let params = mk_map(vec![("a", json!(true)), ("b", Value::Null)]);
            let qs = build_query_string(&params)?;
            assert_eq!(qs, "a=true");
            Ok(())
        }

        #[test]
        fn percent_encode_special_chars() -> Result<()> {
            let params = mk_map(vec![
                ("space", json!("hello world")),
                ("symbols", json!("a/b?c")),
            ]);
            let qs = build_query_string(&params)?;
            let mut parts = vec![];
            let mut ser = Serializer::new(String::new());
            ser.append_pair("space", "hello world");
            parts.push(ser.finish());
            let mut ser = Serializer::new(String::new());
            ser.append_pair("symbols", "a/b?c");
            parts.push(ser.finish());
            let expected = parts.join("&");
            assert_eq!(qs, expected);
            Ok(())
        }

        #[test]
        fn primitive_array_json_encoded() -> Result<()> {
            let params = mk_map(vec![
                ("strs", json!(["a", "b", "c"])),
                ("nums", json!([1, 2, 3])),
                ("bools", json!([true, false])),
            ]);
            let qs = build_query_string(&params)?;

            let mut parts = Vec::new();
            for (k, v) in &params {
                let json = serde_json::to_string(v)?;
                let mut ser = Serializer::new(String::new());
                ser.append_pair(k, &json);
                parts.push(ser.finish());
            }
            let expected = parts.join("&");
            assert_eq!(qs, expected);
            Ok(())
        }

        #[test]
        fn nested_array_json_encoded() -> Result<()> {
            let params = mk_map(vec![("nested", json!([[1, 2], [3, 4]]))]);
            let qs = build_query_string(&params)?;

            let nested_json = serde_json::to_string(&json!([[1, 2], [3, 4]]))?;
            let mut ser = Serializer::new(String::new());
            ser.append_pair("nested", &nested_json);
            let expected = ser.finish();

            assert_eq!(qs, expected);
            Ok(())
        }

        #[test]
        fn object_json_encoded() -> Result<()> {
            let params = mk_map(vec![("obj", json!({"k":1, "v":"two"}))]);
            let qs = build_query_string(&params)?;

            let obj_json = serde_json::to_string(&json!({"k":1, "v":"two"}))?;
            let mut ser = Serializer::new(String::new());
            ser.append_pair("obj", &obj_json);
            let expected = ser.finish();

            assert_eq!(qs, expected);
            Ok(())
        }

        #[test]
        fn empty_array() {
            let params = mk_map(vec![("foo", json!([]))]);
            let qs = build_query_string(&params).unwrap();

            let json = serde_json::to_string(&json!([])).unwrap();
            let expected = Serializer::new(String::new())
                .append_pair("foo", &json)
                .finish();
            assert_eq!(qs, expected);
        }

        #[test]
        fn mixed_array() {
            let params = mk_map(vec![("mix", json!([1, "x", false]))]);
            let qs = build_query_string(&params).unwrap();

            let json = serde_json::to_string(&json!([1, "x", false])).unwrap();
            let expected = Serializer::new(String::new())
                .append_pair("mix", &json)
                .finish();
            assert_eq!(qs, expected);
        }

        #[test]
        fn array_of_objects() {
            let params = mk_map(vec![("objs", json!([{"a":1}, {"b":2}]))]);
            let qs = build_query_string(&params).unwrap();

            let json = serde_json::to_string(&json!([{"a":1}, {"b":2}])).unwrap();
            let expected = Serializer::new(String::new())
                .append_pair("objs", &json)
                .finish();
            assert_eq!(qs, expected);
        }

        #[test]
        fn empty_object() {
            let params = mk_map(vec![("emp", json!({}))]);
            let qs = build_query_string(&params).unwrap();

            let json = serde_json::to_string(&json!({})).unwrap();
            let expected = Serializer::new(String::new())
                .append_pair("emp", &json)
                .finish();
            assert_eq!(qs, expected);
        }

        #[test]
        fn floats_and_negatives() {
            let params = mk_map(vec![("fl", json!(1.23456)), ("neg", json!(-0.001))]);
            let qs = build_query_string(&params).unwrap();
            assert_eq!(qs, "fl=1.23456&neg=-0.001");
        }

        #[test]
        fn unicode_and_special_key() {
            let params = mk_map(vec![
                ("こんにちは", json!("世界")),
                ("weird key/?=", json!("val")),
            ]);
            let qs = build_query_string(&params).unwrap();

            let mut parts = Vec::new();
            for (k, v) in &params {
                let mut ser = Serializer::new(String::new());
                ser.append_pair(k, v.as_str().unwrap());
                parts.push(ser.finish());
            }
            let expected = parts.join("&");
            assert_eq!(qs, expected);
        }

        #[test]
        fn empty_string_value() {
            let params = mk_map(vec![("empty", json!(""))]);
            let qs = build_query_string(&params).unwrap();
            assert_eq!(qs, "empty=");
        }

        #[test]
        fn nulls_in_array() {
            let params = mk_map(vec![("a", json!([null, 1, "x"]))]);
            let qs = build_query_string(&params).unwrap();

            let json = serde_json::to_string(&json!([null, 1, "x"])).unwrap();
            let expected = Serializer::new(String::new())
                .append_pair("a", &json)
                .finish();
            assert_eq!(qs, expected);
        }

        #[test]
        fn special_chars_in_key() {
            let params = mk_map(vec![("a=b&c%", json!("val"))]);
            let qs = build_query_string(&params).unwrap();

            let expected = Serializer::new(String::new())
                .append_pair("a=b&c%", "val")
                .finish();
            assert_eq!(qs, expected);
        }

        #[test]
        fn empty_key() {
            let params = mk_map(vec![("", json!("v"))]);
            let qs = build_query_string(&params).unwrap();
            assert_eq!(qs, "=v");
        }
    }

    mod signature_generator {
        use base64::{Engine, engine::general_purpose};
        use ed25519_dalek::{SigningKey, ed25519::signature::SignerMut, pkcs8::DecodePrivateKey};
        use hex;
        use hmac::{Hmac, Mac};
        use openssl::{hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::Verifier};
        use serde_json::Value;
        use sha2::Sha256;
        use std::collections::BTreeMap;
        use std::io::Write;
        use tempfile::NamedTempFile;

        use crate::{common::utils::SignatureGenerator, config::PrivateKey};

        #[test]
        fn hmac_sha256_signature() {
            let mut params = BTreeMap::new();
            params.insert("b".into(), Value::Number(2.into()));
            params.insert("a".into(), Value::Number(1.into()));

            let signature_gen = SignatureGenerator::new(Some("test-secret".into()), None, None);
            let sig = signature_gen
                .get_signature(&params)
                .expect("HMAC signing failed");

            let mut mac = Hmac::<Sha256>::new_from_slice(b"test-secret").unwrap();
            let qs = "a=1&b=2";
            mac.update(qs.as_bytes());
            let expected = hex::encode(mac.finalize().into_bytes());

            assert_eq!(sig, expected);
        }

        #[test]
        fn repeated_hmac_signature() {
            let mut params = BTreeMap::new();
            params.insert("x".into(), Value::String("y".into()));
            let signature_gen = SignatureGenerator::new(Some("abc".into()), None, None);
            let s1 = signature_gen.get_signature(&params).unwrap();
            let s2 = signature_gen.get_signature(&params).unwrap();
            assert_eq!(s1, s2);
        }

        #[test]
        fn rsa_signature_verification() {
            let mut params = BTreeMap::new();
            params.insert("a".into(), Value::Number(1.into()));
            params.insert("b".into(), Value::Number(2.into()));

            let rsa = Rsa::generate(2048).unwrap();
            let priv_pem = rsa.private_key_to_pem().unwrap();
            let pub_pem = rsa.public_key_to_pem_pkcs1().unwrap();

            let signature_gen =
                SignatureGenerator::new(None, Some(PrivateKey::Raw(priv_pem.clone())), None);
            let sig = signature_gen
                .get_signature(&params)
                .expect("RSA signing failed");

            let sig_bytes = general_purpose::STANDARD.decode(&sig).unwrap();
            let pubkey = PKey::public_key_from_pem(&pub_pem).unwrap();
            let mut verifier = Verifier::new(MessageDigest::sha256(), &pubkey).unwrap();
            verifier.update(b"a=1&b=2").unwrap();
            assert!(verifier.verify(&sig_bytes).unwrap());
        }

        #[test]
        fn repeated_rsa_signature() {
            let mut params = BTreeMap::new();
            params.insert("k".into(), Value::Number(5.into()));
            let rsa = Rsa::generate(2048).unwrap();
            let priv_pem = rsa.private_key_to_pem().unwrap();
            let signature_gen =
                SignatureGenerator::new(None, Some(PrivateKey::Raw(priv_pem)), None);
            let s1 = signature_gen.get_signature(&params).unwrap();
            let s2 = signature_gen.get_signature(&params).unwrap();
            assert_eq!(s1, s2);
        }

        #[test]
        fn ed25519_signature_verification() {
            let mut params = BTreeMap::new();
            params.insert("a".into(), Value::Number(1.into()));
            params.insert("b".into(), Value::Number(2.into()));
            let qs = "a=1&b=2";

            let ed = PKey::generate_ed25519().unwrap();
            let priv_pem = ed.private_key_to_pem_pkcs8().unwrap();

            let signature_gen =
                SignatureGenerator::new(None, Some(PrivateKey::Raw(priv_pem.clone())), None);
            let sig = signature_gen
                .get_signature(&params)
                .expect("Ed25519 signing failed");

            let pem_str = String::from_utf8(priv_pem).unwrap();
            let b64 = pem_str
                .lines()
                .filter(|l| !l.starts_with("-----"))
                .collect::<String>();
            let der = general_purpose::STANDARD.decode(b64).unwrap();
            let mut sk = SigningKey::from_pkcs8_der(&der).unwrap();
            let expected_bytes = sk.sign(qs.as_bytes()).to_bytes();
            let expected_sig = general_purpose::STANDARD.encode(expected_bytes);
            assert_eq!(sig, expected_sig);
        }

        #[test]
        fn repeated_ed25519_signature() {
            let mut params = BTreeMap::new();
            params.insert("m".into(), Value::String("n".into()));
            let ed = PKey::generate_ed25519().unwrap();
            let priv_pem = ed.private_key_to_pem_pkcs8().unwrap();
            let signature_gen =
                SignatureGenerator::new(None, Some(PrivateKey::Raw(priv_pem.clone())), None);
            let s1 = signature_gen.get_signature(&params).unwrap();
            let s2 = signature_gen.get_signature(&params).unwrap();
            assert_eq!(s1, s2);
        }

        #[test]
        fn file_based_key() {
            let rsa = Rsa::generate(1024).unwrap();
            let priv_pem = rsa.private_key_to_pem().unwrap();
            let pub_pem = rsa.public_key_to_pem_pkcs1().unwrap();

            let mut file = NamedTempFile::new().unwrap();
            file.write_all(&priv_pem).unwrap();
            let path = file.path().to_str().unwrap().to_string();

            let mut params = BTreeMap::new();
            params.insert("z".into(), Value::Number(9.into()));

            let signature_gen = SignatureGenerator::new(None, Some(PrivateKey::File(path)), None);
            let sig = signature_gen.get_signature(&params).unwrap();

            let sig_bytes = general_purpose::STANDARD.decode(&sig).unwrap();
            let pubkey = PKey::public_key_from_pem(&pub_pem).unwrap();
            let mut verifier = Verifier::new(MessageDigest::sha256(), &pubkey).unwrap();
            verifier.update(b"z=9").unwrap();
            assert!(verifier.verify(&sig_bytes).unwrap());
        }

        #[test]
        fn unsupported_key_type_error() {
            let mut params = BTreeMap::new();
            params.insert("x".into(), Value::String("y".into()));

            let group =
                openssl::ec::EcGroup::from_curve_name(openssl::nid::Nid::X9_62_PRIME256V1).unwrap();
            let ec_key = openssl::ec::EcKey::generate(&group).unwrap();
            let pkey_ec = PKey::from_ec_key(ec_key).unwrap();
            let raw = pkey_ec.private_key_to_pem_pkcs8().unwrap();

            let signature_gen = SignatureGenerator::new(None, Some(PrivateKey::Raw(raw)), None);
            let err = signature_gen
                .get_signature(&params)
                .unwrap_err()
                .to_string();
            assert!(err.contains("Unsupported private key type"));
        }

        #[test]
        fn invalid_private_key_error() {
            let mut params = BTreeMap::new();
            params.insert("foo".into(), Value::String("bar".into()));

            let signature_gen =
                SignatureGenerator::new(None, Some(PrivateKey::Raw(b"not a key".to_vec())), None);
            let err = signature_gen
                .get_signature(&params)
                .unwrap_err()
                .to_string();
            assert!(err.contains("Failed to parse private key"));
        }

        #[test]
        fn missing_credentials_error() {
            let mut params = BTreeMap::new();
            params.insert("a".into(), Value::Number(1.into()));

            let signature_gen = SignatureGenerator::new(None, None, None);
            let err = signature_gen
                .get_signature(&params)
                .unwrap_err()
                .to_string();
            assert!(err.contains("Either 'api_secret' or 'private_key' must be provided"));
        }
    }

    mod should_retry_request {
        use crate::common::utils::should_retry_request;

        use reqwest::{Error, Response};

        fn mk_http_error(code: u16) -> Error {
            let resp = Response::from(
                http::response::Response::builder()
                    .status(code)
                    .body("")
                    .unwrap(),
            );
            resp.error_for_status().unwrap_err()
        }

        fn mk_network_error() -> Error {
            reqwest::blocking::get("http://256.256.256.256").unwrap_err()
        }

        #[test]
        fn retry_on_retriable_status_and_method() {
            let err = mk_http_error(500);
            assert!(should_retry_request(&err, Some("GET"), Some(1)));
            assert!(should_retry_request(&err, Some("delete"), Some(2)));
        }

        #[test]
        fn retry_when_status_none_and_retriable_method() {
            let retriable_methods = ["GET", "DELETE"];

            for &method in &retriable_methods {
                let err = mk_network_error();
                assert!(
                    should_retry_request(&err, Some(method), Some(1)),
                    "Should retry when no status and method {method}"
                );
            }
        }

        #[test]
        fn no_retry_when_no_retries_left() {
            let err = mk_http_error(503);
            assert!(!should_retry_request(&err, Some("GET"), Some(0)));
        }

        #[test]
        fn no_retry_on_non_retriable_status() {
            let non_retriable_statuses = [400, 401, 404, 422];

            for &status in &non_retriable_statuses {
                let err = mk_http_error(status);
                assert!(
                    !should_retry_request(&err, Some("GET"), Some(2)),
                    "Should not retry for non-retriable status {status}"
                );
            }
        }

        #[test]
        fn no_retry_on_non_retriable_method() {
            let non_retriable_methods = ["POST", "PUT", "PATCH"];

            for &method in &non_retriable_methods {
                let err = mk_http_error(500);
                assert!(
                    !should_retry_request(&err, Some(method), Some(2)),
                    "Should not retry for non-retriable method {method}"
                );
            }
        }

        #[test]
        fn no_retry_when_status_none_and_non_retriable_method() {
            let non_retriable_methods = ["POST", "PUT"];

            for &method in &non_retriable_methods {
                let err = mk_network_error();
                assert!(
                    !should_retry_request(&err, Some(method), Some(1)),
                    "Should not retry when no status and method {method}"
                );
            }
        }
    }

    mod parse_rate_limit_headers_tests {
        use crate::common::{
            models::{Interval, RateLimitType},
            utils::parse_rate_limit_headers,
        };
        use std::collections::HashMap;

        fn mk_headers(pairs: Vec<(&str, &str)>) -> HashMap<String, String> {
            let mut m = HashMap::new();
            for (k, v) in pairs {
                m.insert(k.to_string(), v.to_string());
            }
            m
        }

        #[test]
        fn single_weight_header() {
            let headers = mk_headers(vec![("x-mbx-used-weight-1s", "123")]);
            let limits = parse_rate_limit_headers(&headers);
            assert_eq!(limits.len(), 1);
            let rl = &limits[0];
            assert_eq!(rl.rate_limit_type, RateLimitType::RequestWeight);
            assert_eq!(rl.interval, Interval::Second);
            assert_eq!(rl.interval_num, 1);
            assert_eq!(rl.count, 123);
            assert_eq!(rl.retry_after, None);
        }

        #[test]
        fn single_order_count_with_retry_after() {
            let headers = mk_headers(vec![("x-mbx-order-count-5m", "42"), ("retry-after", "7")]);
            let limits = parse_rate_limit_headers(&headers);
            assert_eq!(limits.len(), 1);
            let rl = &limits[0];
            assert_eq!(rl.rate_limit_type, RateLimitType::Orders);
            assert_eq!(rl.interval, Interval::Minute);
            assert_eq!(rl.interval_num, 5);
            assert_eq!(rl.count, 42);
            assert_eq!(rl.retry_after, Some(7));
        }

        #[test]
        fn multiple_headers() {
            let headers = mk_headers(vec![
                ("X-MBX-USED-WEIGHT-1h", "10"),
                ("x-mbx-order-count-2d", "20"),
            ]);
            let mut limits = parse_rate_limit_headers(&headers);
            limits.sort_by_key(|r| (r.interval_num, format!("{:?}", r.rate_limit_type)));
            assert_eq!(limits.len(), 2);
            let w = &limits[0];
            assert_eq!(w.rate_limit_type, RateLimitType::RequestWeight);
            assert_eq!(w.interval, Interval::Hour);
            assert_eq!(w.interval_num, 1);
            assert_eq!(w.count, 10);
            let o = &limits[1];
            assert_eq!(o.rate_limit_type, RateLimitType::Orders);
            assert_eq!(o.interval, Interval::Day);
            assert_eq!(o.interval_num, 2);
            assert_eq!(o.count, 20);
        }

        #[test]
        fn ignores_unknown_and_malformed() {
            let headers = mk_headers(vec![
                ("x-mbx-used-weight-3x", "5"),
                ("random-header", "100"),
            ]);
            let limits = parse_rate_limit_headers(&headers);
            assert!(limits.is_empty());
        }
    }

    mod http_request {
        use std::io::Write;

        use flate2::{Compression, write::GzEncoder};
        use httpmock::MockServer;
        use reqwest::{Client, Method, Request};
        use serde::Deserialize;

        use crate::{
            common::utils::http_request, config::ConfigurationRestApi, errors::ConnectorError,
            models::RestApiResponse,
        };

        use super::TOKIO_SHARED_RT;

        #[derive(Deserialize, Debug, PartialEq)]
        struct Dummy {
            foo: String,
        }

        fn make_config(server_url: &str) -> ConfigurationRestApi {
            ConfigurationRestApi::builder()
                .api_key("key")
                .api_secret("secret")
                .base_path(server_url)
                .build()
                .expect("Failed to build configuration")
        }

        #[test]
        fn http_request_success_plain_text() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/test");
                    then.status(200)
                        .header("Content-Type", "application/json")
                        .body(r#"{"foo":"bar"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/test"))
                    .build()
                    .unwrap();

                let cfg = make_config(&server.url(""));
                let resp: RestApiResponse<Dummy> = http_request(req, &cfg).await.unwrap();
                assert_eq!(resp.status, 200);
                let data = resp.data().await.unwrap();
                assert_eq!(data, Dummy { foo: "bar".into() });
                mock.assert();
            });
        }

        #[test]
        fn http_request_success_gzip() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let body = r#"{"foo":"baz"}"#;
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(body.as_bytes()).unwrap();
                let gz = encoder.finish().unwrap();

                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/gz");
                    then.status(200)
                        .header("Content-Type", "application/json")
                        .header("Content-Encoding", "gzip")
                        .body(gz);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/gz"))
                    .build()
                    .unwrap();
                let mut cfg = make_config(&server.url(""));
                cfg.compression = true;

                let resp: RestApiResponse<Dummy> = http_request(req, &cfg).await.unwrap();
                assert_eq!(resp.status, 200);
                let data = resp.data().await.unwrap();
                assert_eq!(data, Dummy { foo: "baz".into() });
                mock.assert();
            });
        }

        #[test]
        fn http_request_client_error_bad_request() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/400");
                    then.status(400)
                        .header("Content-Type", "application/json")
                        .body(r#"{"msg":"bad request"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/400"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(result, Err(ConnectorError::BadRequestError(_))));
                if let Err(ConnectorError::BadRequestError(msg)) = result {
                    assert_eq!(msg, "bad request");
                }
                mock.assert();
            });
        }

        #[test]
        fn http_request_client_error_unauthorized() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/401");
                    then.status(401)
                        .header("Content-Type", "application/json")
                        .body(r#"{"msg":"unauthorized"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/401"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(result, Err(ConnectorError::UnauthorizedError(_))));
                if let Err(ConnectorError::UnauthorizedError(msg)) = result {
                    assert_eq!(msg, "unauthorized");
                }
                mock.assert();
            });
        }

        #[test]
        fn http_request_client_error_forbidden() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/403");
                    then.status(403)
                        .header("Content-Type", "application/json")
                        .body(r#"{"msg":"forbidden"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/403"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(result, Err(ConnectorError::ForbiddenError(_))));
                if let Err(ConnectorError::ForbiddenError(msg)) = result {
                    assert_eq!(msg, "forbidden");
                }
                mock.assert();
            });
        }

        #[test]
        fn http_request_client_error_not_found() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/404");
                    then.status(404)
                        .header("Content-Type", "application/json")
                        .body(r#"{"msg":"not found"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/404"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(result, Err(ConnectorError::NotFoundError(_))));
                if let Err(ConnectorError::NotFoundError(msg)) = result {
                    assert_eq!(msg, "not found");
                }
                mock.assert();
            });
        }

        #[test]
        fn http_request_client_error_rate_limit_exceeded() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/418");
                    then.status(418)
                        .header("Content-Type", "application/json")
                        .body(r#"{"msg":"rate limit exceeded"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/418"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(result, Err(ConnectorError::RateLimitBanError(_))));
                if let Err(ConnectorError::RateLimitBanError(msg)) = result {
                    assert_eq!(msg, "rate limit exceeded");
                }
                mock.assert();
            });
        }

        #[test]
        fn http_request_client_error_too_many_requests() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/429");
                    then.status(429)
                        .header("Content-Type", "application/json")
                        .body(r#"{"msg":"too many requests"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/429"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(
                    result,
                    Err(ConnectorError::TooManyRequestsError(_))
                ));
                if let Err(ConnectorError::TooManyRequestsError(msg)) = result {
                    assert_eq!(msg, "too many requests");
                }
                mock.assert();
            });
        }

        #[test]
        fn http_request_client_error_server_error() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/500");
                    then.status(500)
                        .header("Content-Type", "application/json")
                        .body(r#"{"msg":"internal server error"}"#);
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/500"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(result, Err(ConnectorError::ServerError { .. })));
                if let Err(ConnectorError::ServerError {
                    msg,
                    status_code: Some(500),
                }) = result
                {
                    assert_eq!(msg, "Server error: 500".to_string());
                }
                mock.assert();
            });
        }

        #[test]
        fn http_request_unexpected_status_maps_generic() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let code = 402;
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/402");
                    then.status(code).body("error text");
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/402"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                let result = http_request::<Dummy>(req, &cfg).await;
                assert!(matches!(
                    result,
                    Err(ConnectorError::ConnectorClientError(_))
                ));
                mock.assert();
            });
        }

        #[test]
        fn http_request_malformed_json_maps_generic() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();
                let mock = server.mock(|when, then| {
                    when.method(httpmock::Method::GET).path("/malformed");
                    then.status(200)
                        .header("Content-Type", "application/json")
                        .body("not json");
                });

                let client = Client::new();
                let req: Request = client
                    .request(Method::GET, format!("{}{}", server.url(""), "/malformed"))
                    .build()
                    .unwrap();
                let cfg = make_config(&server.url(""));

                // 1) HTTP layer still “succeeds”:
                let resp = http_request::<Dummy>(req, &cfg)
                    .await
                    .expect("http_request should succeed even if JSON is bad");

                // 2) only when we call `.data().await` do we hit the parse‐error:
                let err = resp
                    .data() // or however you invoke that boxed future
                    .await
                    .expect_err("malformed JSON should turn into ConnectorClientError");

                assert!(matches!(err, ConnectorError::ConnectorClientError(_)));

                mock.assert();
            });
        }
    }

    mod send_request {
        use anyhow::Result;
        use httpmock::prelude::*;
        use reqwest::Method;
        use serde::Deserialize;
        use serde_json::json;
        use std::collections::{BTreeMap, HashMap};

        use crate::{
            common::{models::TimeUnit, utils::send_request},
            config::ConfigurationRestApi,
        };

        use super::TOKIO_SHARED_RT;

        #[derive(Deserialize, Debug, PartialEq)]
        struct TestResponse {
            message: String,
        }

        #[test]
        fn basic_get_request() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(GET).path("/api/v1/test");
                    then.status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"message": "success"}"#);
                });

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .build()
                    .expect("Failed to build configuration");

                let params = BTreeMap::new();

                let result = send_request::<TestResponse>(
                    &configuration,
                    "/api/v1/test",
                    Method::GET,
                    params,
                    None,
                    false,
                )
                .await?;

                let data = result.data().await.unwrap();
                assert_eq!(data.message, "success");

                Ok(())
            })
        }

        #[test]
        fn signed_post_request() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(POST).path("/api/v3/order");
                    then.status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"message": "order placed"}"#);
                });

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .build()
                    .expect("Failed to build configuration");

                let mut params = BTreeMap::new();
                params.insert("symbol".to_string(), json!("ETHUSDT"));
                params.insert("side".to_string(), json!("BUY"));
                params.insert("type".to_string(), json!("MARKET"));
                params.insert("quantity".to_string(), json!("1"));

                let result = send_request::<TestResponse>(
                    &configuration,
                    "/api/v3/order",
                    Method::POST,
                    params,
                    None,
                    true,
                )
                .await?;

                let data = result.data().await.unwrap();
                assert_eq!(data.message, "order placed");

                Ok(())
            })
        }

        #[test]
        fn get_request_with_params() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(GET)
                        .path("/api/v1/data")
                        .query_param("symbol", "BTCUSDT")
                        .query_param("limit", "10");
                    then.status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"message": "data retrieved"}"#);
                });

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .build()
                    .expect("Failed to build configuration");

                let mut params = BTreeMap::new();
                params.insert("symbol".to_string(), json!("BTCUSDT"));
                params.insert("limit".to_string(), json!(10));

                let result = send_request::<TestResponse>(
                    &configuration,
                    "/api/v1/data",
                    Method::GET,
                    params,
                    None,
                    false,
                )
                .await?;

                let data = result.data().await.unwrap();
                assert_eq!(data.message, "data retrieved");

                Ok(())
            })
        }

        #[test]
        fn invalid_endpoint() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .build()
                    .expect("Failed to build configuration");

                let params = BTreeMap::new();

                let result = send_request::<TestResponse>(
                    &configuration,
                    "http://invalid",
                    Method::GET,
                    params,
                    None,
                    false,
                )
                .await;

                assert!(result.is_err());
            });
        }

        #[test]
        fn missing_signature_on_signed_request() {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .build()
                    .expect("Failed to build configuration");

                let mut params = BTreeMap::new();
                params.insert("symbol".to_string(), json!("BTCUSDT"));
                params.insert("side".to_string(), json!("BUY"));

                let result = send_request::<TestResponse>(
                    &configuration,
                    "/api/v3/order",
                    Method::POST,
                    params,
                    None,
                    true,
                )
                .await;

                assert!(result.is_err());
            });
        }

        #[test]
        fn compression_enabled() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(GET).path("/api/v1/test");
                    then.status(200)
                        .header("content-type", "application/json")
                        .header("accept-encoding", "gzip, deflate, br")
                        .body(r#"{"message": "compression enabled"}"#);
                });

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(true)
                    .build()
                    .expect("Failed to build configuration");

                let params = BTreeMap::new();

                let result = send_request::<TestResponse>(
                    &configuration,
                    "/api/v1/test",
                    Method::GET,
                    params,
                    None,
                    false,
                )
                .await?;

                let data = result.data().await.unwrap();
                assert_eq!(data.message, "compression enabled");

                Ok(())
            })
        }

        #[test]
        fn get_request_with_time_unit_header() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(GET)
                        .path("/api/v1/test")
                        .header("X-MBX-TIME-UNIT", "MILLISECOND");
                    then.status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"message": "time unit applied"}"#);
                });

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .time_unit(TimeUnit::Millisecond)
                    .build()
                    .expect("Failed to build configuration");

                let params = BTreeMap::new();

                let result = send_request::<TestResponse>(
                    &configuration,
                    "/api/v1/test",
                    Method::GET,
                    params,
                    Some(TimeUnit::Millisecond),
                    false,
                )
                .await?;

                let data = result.data().await.unwrap();
                assert_eq!(data.message, "time unit applied");

                Ok(())
            })
        }

        #[test]
        fn custom_headers_are_sent() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(GET)
                        .path("/api/v1/test")
                        .header("X-My-Test", "all-clear");
                    then.status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"message":"ok"}"#);
                });

                let mut custom = HashMap::new();
                custom.insert("X-My-Test".to_string(), "all-clear".to_string());

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .custom_headers(custom)
                    .build()
                    .expect("Failed to build configuration");

                let params = BTreeMap::new();
                let res = send_request::<TestResponse>(
                    &configuration,
                    "/api/v1/test",
                    Method::GET,
                    params,
                    None,
                    false,
                )
                .await?;

                let data = res.data().await.unwrap();
                assert_eq!(data.message, "ok");

                Ok(())
            })
        }

        #[test]
        fn custom_header_override_prevention() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(GET)
                        .path("/api/v1/test")
                        .header("content-type", "application/json")
                        .header("x-mbx-apikey", "key")
                        .header("X-My-Test", "ok");
                    then.status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"message":"defaults intact"}"#);
                });

                let mut custom = HashMap::new();
                custom.insert("Content-Type".to_string(), "text/plain".to_string());
                custom.insert("X-MBX-APIKEY".to_string(), "BAD".to_string());
                custom.insert("X-My-Test".to_string(), "ok".to_string());

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .custom_headers(custom)
                    .build()
                    .expect("Failed to build configuration");

                let params = BTreeMap::new();
                let res = send_request::<TestResponse>(
                    &configuration,
                    "/api/v1/test",
                    Method::GET,
                    params,
                    None,
                    false,
                )
                .await?;

                let data = res.data().await.unwrap();
                assert_eq!(data.message, "defaults intact");

                Ok(())
            })
        }

        #[test]
        fn crlf_in_header_values_are_dropped() -> Result<()> {
            TOKIO_SHARED_RT.block_on(async {
                let server = MockServer::start();

                server.mock(|when, then| {
                    when.method(GET)
                        .path("/api/v1/test")
                        .header("X-Good", "safe");
                    then.status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"message":"clean only"}"#);
                });

                let mut custom = HashMap::new();
                custom.insert("X-Bad".to_string(), "evil\r\ninject".to_string());
                custom.insert("X-Good".to_string(), "safe".to_string());

                let configuration = ConfigurationRestApi::builder()
                    .api_key("key")
                    .api_secret("secret")
                    .base_path(server.base_url())
                    .compression(false)
                    .custom_headers(custom)
                    .build()
                    .expect("Failed to build configuration");

                let params = BTreeMap::new();
                let res = send_request::<TestResponse>(
                    &configuration,
                    "/api/v1/test",
                    Method::GET,
                    params,
                    None,
                    false,
                )
                .await?;

                let data = res.data().await.unwrap();
                assert_eq!(data.message, "clean only");

                Ok(())
            })
        }
    }

    mod random_string {
        use crate::common::utils::random_string;
        use hex;

        #[test]
        fn length_is_32() {
            let s = random_string();
            assert_eq!(
                s.len(),
                32,
                "random_string() should be 32 chars, got {}",
                s.len()
            );
        }

        #[test]
        fn is_valid_lowercase_hex() {
            let s = random_string();
            assert!(
                s.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
                "random_string() contains invalid hex characters: {s}"
            );
        }

        #[test]
        fn decodes_to_16_bytes() {
            let s = random_string();
            let bytes = hex::decode(&s).expect("random_string() output must be valid hex");
            assert_eq!(
                bytes.len(),
                16,
                "hex::decode returned {} bytes",
                bytes.len()
            );
        }

        #[test]
        fn two_calls_are_different() {
            let a = random_string();
            let b = random_string();
            assert_ne!(
                a, b,
                "Two calls to random_string() returned the same value: {a}"
            );
        }
    }

    mod remove_empty_value {
        use crate::common::utils::remove_empty_value;
        use serde_json::{Map, Value};

        #[test]
        fn filters_out_null_and_empty_strings() {
            let entries = vec![
                ("key1".to_string(), Value::String("value1".to_string())),
                ("key2".to_string(), Value::Null),
                ("key3".to_string(), Value::String(String::new())),
            ];
            let result = remove_empty_value(entries);
            assert_eq!(
                result.len(),
                1,
                "expected only one entry, got {}",
                result.len()
            );
            assert_eq!(
                result.get("key1"),
                Some(&Value::String("value1".to_string()))
            );
            assert!(!result.contains_key("key2"));
            assert!(!result.contains_key("key3"));
        }

        #[test]
        fn retains_other_value_types() {
            let entries = vec![
                ("bool".to_string(), Value::Bool(true)),
                ("num".to_string(), Value::Number(42.into())),
                ("arr".to_string(), Value::Array(vec![])),
                ("obj".to_string(), Value::Object(Map::default())),
                ("nil".to_string(), Value::Null),
                ("empty_str".to_string(), Value::String(String::new())),
            ];
            let result = remove_empty_value(entries);
            let keys: Vec<&String> = result.keys().collect();
            assert_eq!(keys.len(), 4, "expected 4 entries, got {}", keys.len());
            assert!(result.get("bool") == Some(&Value::Bool(true)));
            assert!(result.get("num") == Some(&Value::Number(42.into())));
            assert!(result.get("arr") == Some(&Value::Array(vec![])));
            assert!(result.get("obj") == Some(&Value::Object(Map::default())));
            assert!(!result.contains_key("nil"));
            assert!(!result.contains_key("empty_str"));
        }

        #[test]
        fn empty_iterator_returns_empty_map() {
            let entries: Vec<(String, Value)> = vec![];
            let result = remove_empty_value(entries);
            assert!(result.is_empty(), "expected an empty map");
        }

        #[test]
        fn keys_are_sorted() {
            let entries = vec![
                ("c".to_string(), Value::String("foo".to_string())),
                ("a".to_string(), Value::String("bar".to_string())),
                ("b".to_string(), Value::String("baz".to_string())),
            ];
            let result = remove_empty_value(entries);
            let sorted_keys: Vec<&String> = result.keys().collect();
            assert_eq!(
                sorted_keys,
                [&"a".to_string(), &"b".to_string(), &"c".to_string()]
            );
        }
    }

    mod sort_object_params {
        use crate::common::utils::sort_object_params;
        use serde_json::Value;
        use std::collections::BTreeMap;

        #[test]
        fn sorts_keys() {
            let mut params = BTreeMap::new();
            params.insert("z".to_string(), Value::String("last".to_string()));
            params.insert("a".to_string(), Value::String("first".to_string()));
            params.insert("m".to_string(), Value::String("middle".to_string()));

            let sorted = sort_object_params(&params);
            let keys: Vec<&String> = sorted.keys().collect();
            assert_eq!(
                keys,
                [&"a".to_string(), &"m".to_string(), &"z".to_string()],
                "Keys should be sorted alphabetically"
            );
        }

        #[test]
        fn preserves_values() {
            let mut params = BTreeMap::new();
            params.insert("one".to_string(), Value::Number(1.into()));
            params.insert("two".to_string(), Value::Bool(true));

            let sorted = sort_object_params(&params);
            assert_eq!(sorted.get("one"), Some(&Value::Number(1.into())));
            assert_eq!(sorted.get("two"), Some(&Value::Bool(true)));
        }

        #[test]
        fn empty_map_returns_empty() {
            let params: BTreeMap<String, Value> = BTreeMap::new();
            let sorted = sort_object_params(&params);
            assert!(sorted.is_empty(), "Expected empty map");
        }

        #[test]
        fn independent_clone() {
            let mut params = BTreeMap::new();
            params.insert("key".to_string(), Value::String("val".to_string()));

            let mut sorted = sort_object_params(&params);
            sorted.insert("new".to_string(), Value::String("x".to_string()));

            assert!(
                !params.contains_key("new"),
                "Original should not be modified when changing sorted"
            );
            assert!(
                sorted.contains_key("new"),
                "Sorted map should reflect its own insertions"
            );
        }
    }

    mod normalize_ws_streams_key {
        use crate::common::utils::normalize_ws_streams_key;

        #[test]
        fn returns_empty_for_empty() {
            assert_eq!(normalize_ws_streams_key(""), "");
        }

        #[test]
        fn already_normalized_stays_same() {
            assert_eq!(normalize_ws_streams_key("streamname"), "streamname");
        }

        #[test]
        fn uppercases_are_lowercased() {
            assert_eq!(normalize_ws_streams_key("MyStream"), "mystream");
        }

        #[test]
        fn underscores_are_removed() {
            assert_eq!(normalize_ws_streams_key("my_stream_name"), "mystreamname");
        }

        #[test]
        fn hyphens_are_removed() {
            assert_eq!(normalize_ws_streams_key("my-stream-name"), "mystreamname");
        }

        #[test]
        fn mixed_underscores_and_hyphens_and_case() {
            let input = "Mixed_Case-Stream_Name";
            let expected = "mixedcasestreamname";
            assert_eq!(normalize_ws_streams_key(input), expected);
        }

        #[test]
        fn retains_other_punctuation() {
            assert_eq!(normalize_ws_streams_key("stream.name!"), "stream.name!");
        }
    }

    mod replace_websocket_streams_placeholders {
        use crate::common::utils::replace_websocket_streams_placeholders;
        use std::collections::HashMap;

        #[test]
        fn empty_string_unchanged() {
            let vars: HashMap<&str, &str> = HashMap::new();
            assert_eq!(replace_websocket_streams_placeholders("", &vars), "");
        }

        #[test]
        fn unknown_placeholder_becomes_empty() {
            let vars: HashMap<&str, &str> = HashMap::new();
            assert_eq!(replace_websocket_streams_placeholders("<foo>", &vars), "");
        }

        #[test]
        fn leading_slash_symbol_lowercases_head() {
            let mut vars = HashMap::new();
            vars.insert("symbol", "BTC");
            assert_eq!(
                replace_websocket_streams_placeholders("/<symbol>", &vars),
                "btc"
            );
        }

        #[test]
        fn no_lowercase_without_slash() {
            let mut vars = HashMap::new();
            vars.insert("symbol", "BTC");
            assert_eq!(
                replace_websocket_streams_placeholders("<symbol>", &vars),
                "BTC"
            );
        }

        #[test]
        fn multiple_placeholders_mid_preserve_ats() {
            let mut vars = HashMap::new();
            vars.insert("symbol", "BNBUSDT");
            vars.insert("levels", "10");
            vars.insert("updateSpeed", "1000ms");
            let out = replace_websocket_streams_placeholders(
                "/<symbol>@depth<levels>@<updateSpeed>",
                &vars,
            );
            assert_eq!(out, "bnbusdt@depth10@1000ms");
        }

        #[test]
        fn trailing_at_removed_when_missing_var() {
            let mut vars = HashMap::new();
            vars.insert("symbol", "BNBUSDT");
            vars.insert("levels", "10");
            let out = replace_websocket_streams_placeholders(
                "/<symbol>@depth<levels>@<updateSpeed>",
                &vars,
            );
            assert_eq!(out, "bnbusdt@depth10");
        }

        #[test]
        fn custom_key_normalization_and_value() {
            let mut vars = HashMap::new();
            vars.insert("my-stream_key", "Value");
            assert_eq!(
                replace_websocket_streams_placeholders("<My_Stream-Key>", &vars),
                "Value"
            );
        }

        #[test]
        fn text_surrounding_placeholders_intact() {
            let mut vars = HashMap::new();
            vars.insert("symbol", "ABC");
            let input = "pre-<symbol>-post";
            assert_eq!(
                replace_websocket_streams_placeholders(input, &vars),
                "pre-ABC-post"
            );
        }
    }

    mod build_websocket_api_message {
        use serde_json::{Value, json};
        use std::collections::BTreeMap;

        use crate::{
            common::{
                utils::{ID_REGEX, build_websocket_api_message, remove_empty_value},
                websocket::WebsocketMessageSendOptions,
            },
            config::ConfigurationWebsocketApi,
        };

        fn make_config() -> ConfigurationWebsocketApi {
            ConfigurationWebsocketApi::builder()
                .api_key("api-key".to_string())
                .api_secret("api-secret".to_string())
                .build()
                .unwrap()
        }

        #[test]
        fn no_auth_or_sign_with_skip_auth() {
            let mut payload = BTreeMap::new();
            payload.insert("foo".into(), Value::String("bar".into()));
            let cfg = make_config();

            let (id, req) = build_websocket_api_message(
                &cfg,
                "method",
                payload.clone(),
                &WebsocketMessageSendOptions {
                    with_api_key: true,
                    is_signed: true,
                    ..Default::default()
                },
                true,
            );

            assert!(ID_REGEX.is_match(&id));
            assert_eq!(req["method"], "method");
            assert_eq!(req["params"]["foo"], "bar");
            assert!(req["params"].get("apiKey").is_none());
            assert!(req["params"].get("signature").is_none());
            assert!(req["params"]["timestamp"].is_number());
        }

        #[test]
        fn only_api_key_when_not_signed() {
            let cfg = make_config();

            let (id, req) = build_websocket_api_message(
                &cfg,
                "method",
                BTreeMap::new(),
                &WebsocketMessageSendOptions {
                    with_api_key: true,
                    is_signed: false,
                    ..Default::default()
                },
                false,
            );

            assert!(ID_REGEX.is_match(&id));
            assert_eq!(req["method"], "method");
            assert_eq!(req["params"]["apiKey"], "api-key");
            assert!(req["params"].get("timestamp").is_none());
            assert!(req["params"].get("signature").is_none());
        }

        #[test]
        fn signed_includes_timestamp_and_signature() {
            let mut payload = BTreeMap::new();
            payload.insert("foo".into(), Value::String("bar".into()));
            let cfg = make_config();

            let (id, req) = build_websocket_api_message(
                &cfg,
                "method",
                payload.clone(),
                &WebsocketMessageSendOptions {
                    with_api_key: true,
                    is_signed: true,
                    ..Default::default()
                },
                false,
            );

            assert!(ID_REGEX.is_match(&id));
            assert_eq!(req["method"], "method");

            let params = &req["params"];
            assert_eq!(params["apiKey"], "api-key");

            let timestamp = params["timestamp"].as_i64().unwrap();
            assert!(timestamp > 0, "timestamp should not be empty");

            let sig = params["signature"].as_str().unwrap();
            assert!(!sig.is_empty(), "signature should not be empty");
        }

        #[test]
        fn respects_provided_valid_id_and_removes_from_params() {
            let mut payload = BTreeMap::new();
            let custom = "0123456789abcdef0123456789abcdef".to_string();
            payload.insert("id".into(), Value::String(custom.clone()));
            payload.insert("foo".into(), Value::Number(123.into()));

            let cfg = make_config();
            let (id, req) = build_websocket_api_message(
                &cfg,
                "method",
                payload.clone(),
                &WebsocketMessageSendOptions::default(),
                true,
            );

            assert_eq!(id, custom);
            assert!(req["params"].get("id").is_none());
            assert_eq!(req["params"]["foo"], 123);
        }

        #[test]
        fn skip_auth_blocks_api_and_signature_but_keeps_timestamp() {
            let mut payload = BTreeMap::new();
            payload.insert("foo".into(), Value::String("bar".into()));
            let cfg = make_config();

            let (_id, req) = build_websocket_api_message(
                &cfg,
                "method",
                payload.clone(),
                &WebsocketMessageSendOptions {
                    with_api_key: true,
                    is_signed: true,
                    ..Default::default()
                },
                true,
            );

            let p = &req["params"];
            assert_eq!(p["foo"], "bar");
            assert!(p.get("apiKey").is_none());
            assert!(p.get("signature").is_none());
            assert!(p["timestamp"].is_number());
        }

        #[test]
        fn random_id_changes_each_call() {
            let cfg = make_config();
            let (id1, _) = build_websocket_api_message(
                &cfg,
                "method",
                BTreeMap::new(),
                &WebsocketMessageSendOptions::default(),
                true,
            );
            let (id2, _) = build_websocket_api_message(
                &cfg,
                "method",
                BTreeMap::new(),
                &WebsocketMessageSendOptions::default(),
                true,
            );
            assert!(ID_REGEX.is_match(&id1));
            assert!(ID_REGEX.is_match(&id2));
            assert_ne!(id1, id2, "IDs should be random and not equal");
        }

        #[test]
        fn null_and_empty_values_are_stripped() {
            let mut payload = BTreeMap::new();
            payload.insert("a".into(), Value::Null);
            payload.insert("b".into(), Value::String(String::new()));
            payload.insert("c".into(), Value::String("ok".into()));

            let cleaned = remove_empty_value(payload.clone());
            assert!(!cleaned.contains_key("a"), "Null should be stripped");
            assert!(
                !cleaned.contains_key("b"),
                "Empty string should be stripped"
            );
            assert!(cleaned.contains_key("c"), "Non-empty string should be kept");

            let cfg = make_config();
            let (_id, req) = build_websocket_api_message(
                &cfg,
                "method",
                payload,
                &WebsocketMessageSendOptions::default(),
                true,
            );
            let params = &req["params"];
            assert!(params.get("a").is_none(), "`a` should not appear");
            assert!(params.get("b").is_none(), "`b` should not appear");
            assert_eq!(params["c"], "ok", "`c` should be present with value \"ok\"");
        }

        #[test]
        fn provided_invalid_id_gets_replaced() {
            let mut payload = BTreeMap::new();
            payload.insert("id".into(), Value::String("not-hex-32-chars".into()));
            let cfg = make_config();
            let (id, _req) = build_websocket_api_message(
                &cfg,
                "method",
                payload,
                &WebsocketMessageSendOptions::default(),
                true,
            );

            assert!(ID_REGEX.is_match(&id));
            assert_ne!(id, "not-hex-32-chars");
        }

        #[test]
        fn sign_only_includes_api_key_even_when_with_api_key_false() {
            let mut payload = BTreeMap::new();
            payload.insert("x".into(), json!(1));

            let cfg = make_config();
            let (_id, req) = build_websocket_api_message(
                &cfg,
                "method",
                payload,
                &WebsocketMessageSendOptions {
                    with_api_key: false,
                    is_signed: true,
                    ..Default::default()
                },
                false,
            );
            let params = &req["params"];

            assert_eq!(params["apiKey"], "api-key");
            assert!(params["timestamp"].is_number());
            assert!(params["signature"].is_string());
        }

        #[test]
        fn skip_auth_false_without_any_auth_flags() {
            let cfg = make_config();
            let (_id, req) = build_websocket_api_message(
                &cfg,
                "method",
                BTreeMap::new(),
                &WebsocketMessageSendOptions {
                    with_api_key: false,
                    is_signed: false,
                    ..Default::default()
                },
                false,
            );
            let params = &req["params"];
            assert!(params.as_object().unwrap().is_empty());
        }
    }
}
