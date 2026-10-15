pub mod rest_api;

pub mod websocket_streams;

use crate::common::{
    config::{ConfigurationRestApi, ConfigurationWebsocketStreams},
    constants::{W3W_PREDICTION_REST_API_PROD_URL, W3W_PREDICTION_WS_STREAMS_PROD_URL},
    utils::build_user_agent,
};

/// Represents the `W3WPrediction` REST API client for interacting with the Binance `W3WPrediction` REST API.
///
/// This struct provides methods to create REST API clients for production  environments.
pub struct W3WPredictionRestApi {}

impl W3WPredictionRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production `W3WPrediction` REST API URL.
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
        config.user_agent = build_user_agent("w3w-prediction");
        if config.base_path.is_none() {
            config.base_path = Some(W3W_PREDICTION_REST_API_PROD_URL.to_string());
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
        config.base_path = Some(W3W_PREDICTION_REST_API_PROD_URL.to_string());
        W3WPredictionRestApi::from_config(config)
    }
}

/// Represents the `W3WPrediction` WebSocket Streams client for interacting with the Binance `W3WPrediction` WebSocket Streams.
///
/// This struct provides methods to create WebSocket Streams clients for production  environments.
pub struct W3WPredictionWsStreams {}

impl W3WPredictionWsStreams {
    /// Creates a WebSocket streams client configured with the given settings.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production `W3WPrediction` WebSocket Streams URL.
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
        config.user_agent = build_user_agent("w3w-prediction");
        if config.ws_url.is_none() {
            config.ws_url = Some(W3W_PREDICTION_WS_STREAMS_PROD_URL.to_string());
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
        config.ws_url = Some(W3W_PREDICTION_WS_STREAMS_PROD_URL.to_string());
        W3WPredictionWsStreams::from_config(config)
    }
}
