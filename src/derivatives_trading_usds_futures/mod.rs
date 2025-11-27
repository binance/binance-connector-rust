pub mod rest_api;
pub mod websocket_api;
pub mod websocket_streams;

use crate::common::{
    config::{ConfigurationRestApi, ConfigurationWebsocketApi, ConfigurationWebsocketStreams},
    constants::{
        DERIVATIVES_TRADING_USDS_FUTURES_REST_API_PROD_URL,
        DERIVATIVES_TRADING_USDS_FUTURES_REST_API_TESTNET_URL,
        DERIVATIVES_TRADING_USDS_FUTURES_WS_API_PROD_URL,
        DERIVATIVES_TRADING_USDS_FUTURES_WS_API_TESTNET_URL,
        DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_PROD_URL,
        DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_TESTNET_URL,
    },
    utils::build_user_agent,
};

/// Represents the `DerivativesTradingUsdsFutures` REST API client for interacting with the Binance `DerivativesTradingUsdsFutures` REST API.
///
/// This struct provides methods to create REST API clients for both production and testnet environments.
pub struct DerivativesTradingUsdsFuturesRestApi {}

impl DerivativesTradingUsdsFuturesRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production `DerivativesTradingUsdsFutures` REST API URL.
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
        config.user_agent = build_user_agent("derivatives-trading-usds-futures");
        if config.base_path.is_none() {
            config.base_path = Some(DERIVATIVES_TRADING_USDS_FUTURES_REST_API_PROD_URL.to_string());
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
        config.base_path = Some(DERIVATIVES_TRADING_USDS_FUTURES_REST_API_PROD_URL.to_string());
        DerivativesTradingUsdsFuturesRestApi::from_config(config)
    }

    /// Creates a REST API client configured for the testnet environment.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the REST API client
    ///
    /// # Returns
    ///
    /// A new REST API client configured for the testnet environment
    #[must_use]
    pub fn testnet(mut config: ConfigurationRestApi) -> rest_api::RestApi {
        config.base_path = Some(DERIVATIVES_TRADING_USDS_FUTURES_REST_API_TESTNET_URL.to_string());
        DerivativesTradingUsdsFuturesRestApi::from_config(config)
    }
}

/// Represents the `DerivativesTradingUsdsFutures` WebSocket API client for interacting with the Binance `DerivativesTradingUsdsFutures` WebSocket API.
///
/// This struct provides methods to create WebSocket API clients for both production and testnet environments.
pub struct DerivativesTradingUsdsFuturesWsApi {}

impl DerivativesTradingUsdsFuturesWsApi {
    /// Creates a WebSocket API client with the given configuration.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production `DerivativesTradingUsdsFutures` WebSocket API URL.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the WebSocket API client
    ///
    /// # Returns
    ///
    /// A new WebSocket API client configured with the provided settings
    #[must_use]
    pub fn from_config(mut config: ConfigurationWebsocketApi) -> websocket_api::WebsocketApiHandle {
        config.user_agent = build_user_agent("derivatives-trading-usds-futures");
        if config.ws_url.is_none() {
            config.ws_url = Some(DERIVATIVES_TRADING_USDS_FUTURES_WS_API_PROD_URL.to_string());
        }
        websocket_api::WebsocketApiHandle::new(config)
    }

    /// Creates a WebSocket API client configured for the production environment.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the WebSocket API client
    ///
    /// # Returns
    ///
    /// A new WebSocket API client configured for the production environment
    #[must_use]
    pub fn production(mut config: ConfigurationWebsocketApi) -> websocket_api::WebsocketApiHandle {
        config.ws_url = Some(DERIVATIVES_TRADING_USDS_FUTURES_WS_API_PROD_URL.to_string());
        DerivativesTradingUsdsFuturesWsApi::from_config(config)
    }

    /// Creates a WebSocket API client configured for the testnet environment.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the WebSocket API client
    ///
    /// # Returns
    ///
    /// A new WebSocket API client configured for the testnet environment
    #[must_use]
    pub fn testnet(mut config: ConfigurationWebsocketApi) -> websocket_api::WebsocketApiHandle {
        config.ws_url = Some(DERIVATIVES_TRADING_USDS_FUTURES_WS_API_TESTNET_URL.to_string());
        DerivativesTradingUsdsFuturesWsApi::from_config(config)
    }
}

/// Represents the `DerivativesTradingUsdsFutures` WebSocket Streams client for interacting with the Binance `DerivativesTradingUsdsFutures` WebSocket Streams.
///
/// This struct provides methods to create WebSocket Streams clients for both production and testnet environments.
pub struct DerivativesTradingUsdsFuturesWsStreams {}

impl DerivativesTradingUsdsFuturesWsStreams {
    /// Creates a WebSocket streams client configured with the given settings.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production `DerivativesTradingUsdsFutures` WebSocket Streams URL.
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
        config.user_agent = build_user_agent("derivatives-trading-usds-futures");
        if config.ws_url.is_none() {
            config.ws_url = Some(DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_PROD_URL.to_string());
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
        config.ws_url = Some(DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_PROD_URL.to_string());
        DerivativesTradingUsdsFuturesWsStreams::from_config(config)
    }

    /// Creates a WebSocket streams client configured for the testnet environment.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the WebSocket streams client
    ///
    /// # Returns
    ///
    /// A new WebSocket streams client configured for the testnet environment
    #[must_use]
    pub fn testnet(
        mut config: ConfigurationWebsocketStreams,
    ) -> websocket_streams::WebsocketStreamsHandle {
        config.ws_url = Some(DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_TESTNET_URL.to_string());
        DerivativesTradingUsdsFuturesWsStreams::from_config(config)
    }
}
