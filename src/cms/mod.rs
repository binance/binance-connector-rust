pub mod websocket_streams;

use crate::common::{
    config::ConfigurationWebsocketStreams, constants::CMS_WS_STREAMS_PROD_URL,
    utils::build_user_agent,
};

/// Represents the Cms WebSocket Streams client for interacting with the Binance Cms WebSocket Streams.
///
/// This struct provides methods to create WebSocket Streams clients for production  environments.
pub struct CmsWsStreams {}

impl CmsWsStreams {
    /// Creates a WebSocket streams client configured with the given settings.
    ///
    /// If no WS URL is specified in the configuration, defaults to the production Cms WebSocket Streams URL.
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
        config.user_agent = build_user_agent("cms");
        if config.ws_url.is_none() {
            config.ws_url = Some(CMS_WS_STREAMS_PROD_URL.to_string());
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
        config.ws_url = Some(CMS_WS_STREAMS_PROD_URL.to_string());
        CmsWsStreams::from_config(config)
    }
}
