use derive_builder::Builder;
use reqwest::{Client, ClientBuilder};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio_tungstenite::Connector;

use super::models::{ConfigBuildError, TimeUnit, WebsocketMode};
use super::utils::{SignatureGenerator, build_client};

#[derive(Clone)]
pub struct AgentConnector(pub Connector);

impl fmt::Debug for AgentConnector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connector(…)")
    }
}

/// The result produced by a custom [`WebsocketHandshakeFn`]: either the established
/// WebSocket stream together with the server's HTTP handshake response, or a
/// `tungstenite` error describing why the handshake could not be completed.
///
/// This mirrors the return type of
/// [`tokio_tungstenite::connect_async_tls_with_config`], so an implementation can
/// simply delegate to it after doing any transport-level setup it needs (see
/// [`WebsocketHandshakeFn`] for details).
pub type WebsocketHandshakeResult = Result<
    (
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        tokio_tungstenite::tungstenite::handshake::client::Response,
    ),
    tokio_tungstenite::tungstenite::Error,
>;

/// A caller-supplied async function that performs the TLS/WebSocket handshake in
/// place of the SDK's default connection path.
///
/// This is useful when the underlying transport needs to be established before the
/// handshake runs — for example, tunneling the connection through a SOCKS5 proxy or
/// another custom stream — something the SDK does not implement itself. The
/// callback is used for the initial connection, reconnects, and connection renewal,
/// and is subject to the same 10-second timeout and error mapping as the SDK's
/// built-in handshake.
///
/// # Arguments passed to the function
///
/// The function receives the same inputs that
/// [`tokio_tungstenite::connect_async_tls_with_config`] takes, so an implementation
/// can open its own transport and then delegate to that function (or an equivalent)
/// to finish the handshake:
///
/// 1. `Request` — the outgoing HTTP upgrade request (already carries the target URL
///    and any headers the SDK has set, such as `User-Agent`).
/// 2. `Option<WebSocketConfig>` — optional WebSocket protocol configuration.
/// 3. `bool` — whether to disable Nagle's algorithm on the underlying TCP stream.
/// 4. `Option<Connector>` — the TLS connector derived from the configuration's
///    `agent` field, if one was set. A custom handshake function that ignores this
///    value effectively overrides `agent`; forward it to the delegate call if the
///    two options are meant to compose.
///
/// # Relationship with `agent`
///
/// `agent` and `handshake` are independent, composable settings: `agent` only
/// customizes the TLS connector, while `handshake` replaces how the transport is
/// opened and the handshake is performed. Setting both is valid — the resolved
/// `agent` connector is simply passed through as the fourth argument above for the
/// custom handshake function to use as it sees fit.
pub type WebsocketHandshakeFn = Arc<
    dyn Fn(
            tokio_tungstenite::tungstenite::handshake::client::Request,
            Option<tokio_tungstenite::tungstenite::protocol::WebSocketConfig>,
            bool,
            Option<Connector>,
        ) -> std::pin::Pin<Box<dyn Future<Output = WebsocketHandshakeResult> + Send>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct HttpAgent(pub Arc<dyn Fn(ClientBuilder) -> ClientBuilder + Send + Sync>);

impl fmt::Debug for HttpAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpAgent(<custom agent fn>)")
    }
}

#[derive(Clone)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    pub protocol: Option<String>,
    pub auth: Option<ProxyAuth>,
}

impl fmt::Debug for ProxyAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProxyAuth")
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone)]
pub enum PrivateKey {
    File(String),
    Raw(Vec<u8>),
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrivateKey::File(_) => write!(f, "PrivateKey::File([REDACTED])"),
            PrivateKey::Raw(_) => write!(f, "PrivateKey::Raw([REDACTED])"),
        }
    }
}

#[derive(Clone, Builder)]
#[builder(
    pattern = "owned",
    build_fn(name = "try_build", error = "ConfigBuildError")
)]
pub struct ConfigurationRestApi {
    #[builder(setter(into, strip_option), default)]
    pub api_key: Option<String>,

    #[builder(setter(into, strip_option), default)]
    pub api_secret: Option<String>,

    #[builder(setter(into, strip_option), default)]
    pub base_path: Option<String>,

    #[builder(default = "1000")]
    pub timeout: u64,

    #[builder(default = "true")]
    pub keep_alive: bool,

    #[builder(default = "true")]
    pub compression: bool,

    #[builder(default = "3")]
    pub retries: u32,

    #[builder(default = "1000")]
    pub backoff: u64,

    #[builder(setter(strip_option), default)]
    pub proxy: Option<ProxyConfig>,

    #[builder(setter(strip_option, into), default)]
    pub custom_headers: Option<HashMap<String, String>>,

    #[builder(setter(strip_option), default)]
    pub agent: Option<HttpAgent>,

    #[builder(setter(strip_option), default)]
    pub private_key: Option<PrivateKey>,

    #[builder(setter(strip_option), default)]
    pub private_key_passphrase: Option<String>,

    #[builder(setter(strip_option), default)]
    pub time_unit: Option<TimeUnit>,

    #[builder(setter(skip))]
    pub(crate) client: Client,

    #[builder(setter(skip))]
    pub(crate) user_agent: String,

    #[builder(setter(skip))]
    pub(crate) signature_gen: SignatureGenerator,
}

impl fmt::Debug for ConfigurationRestApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigurationRestApi")
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field(
                "api_secret",
                &self.api_secret.as_ref().map(|_| "[REDACTED]"),
            )
            .field("base_path", &self.base_path)
            .field("timeout", &self.timeout)
            .field("keep_alive", &self.keep_alive)
            .field("compression", &self.compression)
            .field("retries", &self.retries)
            .field("backoff", &self.backoff)
            .field("proxy", &self.proxy)
            .field("custom_headers", &self.custom_headers)
            .field("agent", &self.agent)
            .field(
                "private_key",
                &self.private_key.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "private_key_passphrase",
                &self.private_key_passphrase.as_ref().map(|_| "[REDACTED]"),
            )
            .field("time_unit", &self.time_unit)
            .field("client", &"<reqwest::Client>")
            .field("user_agent", &self.user_agent)
            .field("signature_gen", &self.signature_gen)
            .finish()
    }
}

impl ConfigurationRestApi {
    #[must_use]
    pub fn builder() -> ConfigurationRestApiBuilder {
        ConfigurationRestApiBuilder::default()
    }
}

impl ConfigurationRestApiBuilder {
    /// Builds a `ConfigurationRestApi` instance with configured HTTP client and signature generator.
    ///
    /// # Returns
    ///
    /// A `Result` containing the fully configured `ConfigurationRestApi` or a `ConfigBuildError` if configuration fails.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigBuildError` if the initial configuration build fails or if client setup encounters issues.
    pub fn build(self) -> Result<ConfigurationRestApi, ConfigBuildError> {
        let mut cfg = self.try_build()?;
        cfg.client = build_client(
            cfg.timeout,
            cfg.keep_alive,
            cfg.proxy.as_ref(),
            cfg.agent.clone(),
        );
        cfg.signature_gen = SignatureGenerator::new(
            cfg.api_secret.clone(),
            cfg.private_key.clone(),
            cfg.private_key_passphrase.clone(),
        );

        Ok(cfg)
    }
}

#[derive(Clone, Builder)]
#[builder(
    pattern = "owned",
    build_fn(name = "try_build", error = "ConfigBuildError")
)]
pub struct ConfigurationWebsocketApi {
    #[builder(setter(into, strip_option), default)]
    pub api_key: Option<String>,

    #[builder(setter(into, strip_option), default)]
    pub api_secret: Option<String>,

    #[builder(setter(into, strip_option), default)]
    pub ws_url: Option<String>,

    #[builder(default = "5000")]
    pub timeout: u64,

    #[builder(default = "5000")]
    pub reconnect_delay: u64,

    #[builder(default = "WebsocketMode::Single")]
    pub mode: WebsocketMode,

    #[builder(setter(strip_option), default)]
    pub agent: Option<AgentConnector>,

    /// Overrides how the SDK performs the TLS/WebSocket handshake. See
    /// [`WebsocketHandshakeFn`] for details and its interaction with `agent`.
    #[builder(setter(strip_option), default)]
    pub handshake: Option<WebsocketHandshakeFn>,

    #[builder(setter(strip_option), default)]
    pub private_key: Option<PrivateKey>,

    #[builder(setter(strip_option), default)]
    pub private_key_passphrase: Option<String>,

    #[builder(setter(strip_option), default)]
    pub time_unit: Option<TimeUnit>,

    #[builder(default = "true")]
    pub auto_session_relogon: bool,

    #[builder(setter(skip))]
    pub(crate) user_agent: String,

    #[builder(setter(skip))]
    pub(crate) signature_gen: SignatureGenerator,
}

impl fmt::Debug for ConfigurationWebsocketApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigurationWebsocketApi")
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field(
                "api_secret",
                &self.api_secret.as_ref().map(|_| "[REDACTED]"),
            )
            .field("ws_url", &self.ws_url)
            .field("timeout", &self.timeout)
            .field("reconnect_delay", &self.reconnect_delay)
            .field("mode", &self.mode)
            .field("agent", &self.agent)
            .field(
                "handshake",
                &self.handshake.as_ref().map(|_| "<custom handshake fn>"),
            )
            .field(
                "private_key",
                &self.private_key.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "private_key_passphrase",
                &self.private_key_passphrase.as_ref().map(|_| "[REDACTED]"),
            )
            .field("time_unit", &self.time_unit)
            .field("auto_session_relogon", &self.auto_session_relogon)
            .field("user_agent", &self.user_agent)
            .field("signature_gen", &self.signature_gen)
            .finish()
    }
}

impl ConfigurationWebsocketApi {
    /// Creates a builder for `ConfigurationWebsocketApi` with the specified API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to be used for the WebSocket API configuration
    ///
    /// # Returns
    ///
    /// A `ConfigurationWebsocketApiBuilder` initialized with the provided API key
    #[must_use]
    pub fn builder() -> ConfigurationWebsocketApiBuilder {
        ConfigurationWebsocketApiBuilder::default()
    }
}

impl ConfigurationWebsocketApiBuilder {
    /// Builds the `ConfigurationWebsocketApi` with a generated signature generator.
    ///
    /// This method attempts to build the configuration using the builder's settings
    /// and then initializes the signature generator with the API secret, private key,
    /// and private key passphrase.
    ///
    /// # Returns
    ///
    /// A `Result` containing the fully configured `ConfigurationWebsocketApi` or a
    /// `ConfigBuildError` if the build process fails.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigBuildError` if the initial configuration build fails or if signature generation fails.
    ///
    pub fn build(self) -> Result<ConfigurationWebsocketApi, ConfigBuildError> {
        let mut cfg = self.try_build()?;
        cfg.signature_gen = SignatureGenerator::new(
            cfg.api_secret.clone(),
            cfg.private_key.clone(),
            cfg.private_key_passphrase.clone(),
        );

        Ok(cfg)
    }
}

#[derive(Clone, Builder)]
#[builder(pattern = "owned", build_fn(error = "ConfigBuildError"))]
pub struct ConfigurationWebsocketStreams {
    #[builder(setter(into, strip_option), default)]
    pub ws_url: Option<String>,

    #[builder(default = "5000")]
    pub reconnect_delay: u64,

    #[builder(default = "WebsocketMode::Single")]
    pub mode: WebsocketMode,

    #[builder(setter(strip_option), default)]
    pub agent: Option<AgentConnector>,

    /// Overrides how the SDK performs the TLS/WebSocket handshake. See
    /// [`WebsocketHandshakeFn`] for details and its interaction with `agent`.
    #[builder(setter(strip_option), default)]
    pub handshake: Option<WebsocketHandshakeFn>,

    #[builder(setter(strip_option), default)]
    pub time_unit: Option<TimeUnit>,

    #[builder(setter(skip))]
    pub(crate) user_agent: String,
}

impl fmt::Debug for ConfigurationWebsocketStreams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigurationWebsocketStreams")
            .field("ws_url", &self.ws_url)
            .field("reconnect_delay", &self.reconnect_delay)
            .field("mode", &self.mode)
            .field("agent", &self.agent)
            .field(
                "handshake",
                &self.handshake.as_ref().map(|_| "<custom handshake fn>"),
            )
            .field("time_unit", &self.time_unit)
            .field("user_agent", &self.user_agent)
            .finish()
    }
}

impl ConfigurationWebsocketStreams {
    #[must_use]
    /// Creates a builder for `ConfigurationWebsocketStreams` with default settings.
    ///
    /// # Returns
    ///
    /// A `ConfigurationWebsocketStreamsBuilder` initialized with default values
    pub fn builder() -> ConfigurationWebsocketStreamsBuilder {
        ConfigurationWebsocketStreamsBuilder::default()
    }
}
