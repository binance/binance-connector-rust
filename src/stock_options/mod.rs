pub mod rest_api;

pub mod websocket_streams;

use crate::common::{
    config::{ConfigurationRestApi, ConfigurationWebsocketStreams},
    constants::{STOCK_OPTIONS_REST_API_PROD_URL, STOCK_OPTIONS_WS_STREAMS_PROD_URL},
    utils::build_user_agent,
};

/// Represents the `StockOptions` REST API client for interacting with the Binance `StockOptions` REST API.
///
/// This struct provides methods to create REST API clients for production  environments.
pub struct StockOptionsRestApi {}

impl StockOptionsRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production `StockOptions` REST API URL.
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
        config.user_agent = build_user_agent("stock-options");
        if config.base_path.is_none() {
            config.base_path = Some(STOCK_OPTIONS_REST_API_PROD_URL.to_string());
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
        config.base_path = Some(STOCK_OPTIONS_REST_API_PROD_URL.to_string());
        StockOptionsRestApi::from_config(config)
    }
}

/// Represents the `StockOptions` WebSocket Streams client for interacting with the Binance `StockOptions` WebSocket Streams.
///
/// This struct provides methods to create WebSocket Streams clients for production  environments.
pub struct StockOptionsWsStreams {}

impl StockOptionsWsStreams {
    /// Creates a WebSocket streams client configured with the given settings.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production `StockOptions` WebSocket Streams URL.
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
        config.user_agent = build_user_agent("stock-options");
        if config.ws_url.is_none() {
            config.ws_url = Some(STOCK_OPTIONS_WS_STREAMS_PROD_URL.to_string());
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
        config.ws_url = Some(STOCK_OPTIONS_WS_STREAMS_PROD_URL.to_string());
        StockOptionsWsStreams::from_config(config)
    }
}
