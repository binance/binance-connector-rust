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

#[derive(Debug, Clone, Builder)]
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

    #[builder(setter(strip_option), default)]
    pub time_unit: Option<TimeUnit>,

    #[builder(setter(skip))]
    pub(crate) user_agent: String,
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
