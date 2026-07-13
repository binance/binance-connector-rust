pub mod rest_api;

pub mod websocket_streams;

use crate::common::{
    config::{ConfigurationRestApi, ConfigurationWebsocketStreams},
    constants::{ALPHA_REST_API_PROD_URL, ALPHA_WS_STREAMS_PROD_URL},
    utils::build_user_agent,
};

/// Represents the Alpha REST API client for interacting with the Binance Alpha REST API.
///
/// This struct provides methods to create REST API clients for production  environments.
pub struct AlphaRestApi {}

impl AlphaRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production Alpha REST API URL.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the REST API client
    ///
    /// # Returns
    ///
    /// A new REST API client configured with the provided settings
    #[must_use]
    pub fn from_config(mut config: ConfigurationRestApi) -> rest_api::RestApi {
        config.user_agent = build_user_agent("alpha");
        if config.base_path.is_none() {
            config.base_path = Some(ALPHA_REST_API_PROD_URL.to_string());
        }
        rest_api::RestApi::new(config)
    }

    /// Creates a REST API client configured for the production environment.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the REST API client
    ///
    /// # Returns
    ///
    /// A new REST API client configured for the production environment
    #[must_use]
    pub fn production(mut config: ConfigurationRestApi) -> rest_api::RestApi {
        config.base_path = Some(ALPHA_REST_API_PROD_URL.to_string());
        AlphaRestApi::from_config(config)
    }
}

/// Represents the Alpha WebSocket Streams client for interacting with the Binance Alpha WebSocket Streams.
///
/// This struct provides methods to create WebSocket Streams clients for production  environments.
pub struct AlphaWsStreams {}

impl AlphaWsStreams {
    /// Creates a WebSocket streams client configured with the given settings.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production Alpha WebSocket Streams URL.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the WebSocket streams client
    ///
    /// # Returns
    ///
    /// A new WebSocket streams client configured with the provided settings
    #[must_use]
    pub fn from_config(
        mut config: ConfigurationWebsocketStreams,
    ) -> websocket_streams::WebsocketStreamsHandle {
        config.user_agent = build_user_agent("alpha");
        if config.ws_url.is_none() {
            config.ws_url = Some(ALPHA_WS_STREAMS_PROD_URL.to_string());
        }
        websocket_streams::WebsocketStreamsHandle::new(config)
    }

    /// Creates a WebSocket streams client configured for the production environment.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the WebSocket streams client
    ///
    /// # Returns
    ///
    /// A new WebSocket streams client configured for the production environment
    #[must_use]
    pub fn production(
        mut config: ConfigurationWebsocketStreams,
    ) -> websocket_streams::WebsocketStreamsHandle {
        config.ws_url = Some(ALPHA_WS_STREAMS_PROD_URL.to_string());
        AlphaWsStreams::from_config(config)
    }
}
