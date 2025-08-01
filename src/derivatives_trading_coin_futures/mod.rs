pub mod rest_api;
pub mod websocket_api;
pub mod websocket_streams;

use crate::common::{
    config::{ConfigurationRestApi, ConfigurationWebsocketApi, ConfigurationWebsocketStreams},
    constants::{
        DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL,
        DERIVATIVES_TRADING_COIN_FUTURES_REST_API_TESTNET_URL,
        DERIVATIVES_TRADING_COIN_FUTURES_WS_API_PROD_URL,
        DERIVATIVES_TRADING_COIN_FUTURES_WS_API_TESTNET_URL,
        DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_PROD_URL,
        DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_TESTNET_URL,
    },
    logger,
    utils::build_user_agent,
};

/// Represents the `DerivativesTradingCoinFutures` REST API client for interacting with the Binance `DerivativesTradingCoinFutures` REST API.
///
/// This struct provides methods to create REST API clients for both production and testnet environments.
pub struct DerivativesTradingCoinFuturesRestApi {}

impl DerivativesTradingCoinFuturesRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production `DerivativesTradingCoinFutures` REST API URL.
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
        logger::init();

        config.user_agent = build_user_agent("derivatives-trading-coin-futures");
        if config.base_path.is_none() {
            config.base_path = Some(DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL.to_string());
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
        config.base_path = Some(DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL.to_string());
        DerivativesTradingCoinFuturesRestApi::from_config(config)
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
        config.base_path = Some(DERIVATIVES_TRADING_COIN_FUTURES_REST_API_TESTNET_URL.to_string());
        DerivativesTradingCoinFuturesRestApi::from_config(config)
    }
}

/// Represents the `DerivativesTradingCoinFutures` WebSocket API client for interacting with the Binance `DerivativesTradingCoinFutures` WebSocket API.
///
/// This struct provides methods to create WebSocket API clients for both production and testnet environments.
pub struct DerivativesTradingCoinFuturesWsApi {}

impl DerivativesTradingCoinFuturesWsApi {
    /// Creates a WebSocket API client with the given configuration.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production `DerivativesTradingCoinFutures` WebSocket API URL.
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
        logger::init();

        config.user_agent = build_user_agent("derivatives-trading-coin-futures");
        if config.ws_url.is_none() {
            config.ws_url = Some(DERIVATIVES_TRADING_COIN_FUTURES_WS_API_PROD_URL.to_string());
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
        config.ws_url = Some(DERIVATIVES_TRADING_COIN_FUTURES_WS_API_PROD_URL.to_string());
        DerivativesTradingCoinFuturesWsApi::from_config(config)
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
        config.ws_url = Some(DERIVATIVES_TRADING_COIN_FUTURES_WS_API_TESTNET_URL.to_string());
        DerivativesTradingCoinFuturesWsApi::from_config(config)
    }
}

/// Represents the `DerivativesTradingCoinFutures` WebSocket Streams client for interacting with the Binance `DerivativesTradingCoinFutures` WebSocket Streams.
///
/// This struct provides methods to create WebSocket Streams clients for both production and testnet environments.
pub struct DerivativesTradingCoinFuturesWsStreams {}

impl DerivativesTradingCoinFuturesWsStreams {
    /// Creates a WebSocket streams client configured with the given settings.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production `DerivativesTradingCoinFutures` WebSocket Streams URL.
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
        logger::init();

        config.user_agent = build_user_agent("derivatives-trading-coin-futures");
        if config.ws_url.is_none() {
            config.ws_url = Some(DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_PROD_URL.to_string());
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
        config.ws_url = Some(DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_PROD_URL.to_string());
        DerivativesTradingCoinFuturesWsStreams::from_config(config)
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
        config.ws_url = Some(DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_TESTNET_URL.to_string());
        DerivativesTradingCoinFuturesWsStreams::from_config(config)
    }
}
