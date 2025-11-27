pub mod rest_api;

pub mod websocket_streams;

use crate::common::{
    config::{ConfigurationRestApi, ConfigurationWebsocketStreams},
    constants::{MARGIN_TRADING_REST_API_PROD_URL, MARGIN_TRADING_WS_STREAMS_PROD_URL},
    utils::build_user_agent,
};

/// Represents the `MarginTrading` REST API client for interacting with the Binance `MarginTrading` REST API.
///
/// This struct provides methods to create REST API clients for the production environment.
pub struct MarginTradingRestApi {}

impl MarginTradingRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production `MarginTrading` REST API URL.
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
        config.user_agent = build_user_agent("margin-trading");
        if config.base_path.is_none() {
            config.base_path = Some(MARGIN_TRADING_REST_API_PROD_URL.to_string());
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
        config.base_path = Some(MARGIN_TRADING_REST_API_PROD_URL.to_string());
        MarginTradingRestApi::from_config(config)
    }
}

/// Represents the `MarginTrading` WebSocket Streams client for interacting with the Binance `MarginTrading` WebSocket Streams.
///
/// This struct provides methods to create WebSocket Streams clients for the production environment.
pub struct MarginTradingWsStreams {}

impl MarginTradingWsStreams {
    /// Creates a WebSocket streams client configured with the given settings.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production `MarginTrading` WebSocket Streams URL.
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
        config.user_agent = build_user_agent("margin-trading");
        if config.ws_url.is_none() {
            config.ws_url = Some(MARGIN_TRADING_WS_STREAMS_PROD_URL.to_string());
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
        config.ws_url = Some(MARGIN_TRADING_WS_STREAMS_PROD_URL.to_string());
        MarginTradingWsStreams::from_config(config)
    }
}
