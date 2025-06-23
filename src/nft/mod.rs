pub mod rest_api;

use crate::common::{
    config::ConfigurationRestApi, constants::NFT_REST_API_PROD_URL, logger, utils::build_user_agent,
};

/// Represents the NFT REST API client for interacting with the Binance NFT REST API.
///
/// This struct provides methods to create REST API clients for the production environment.
pub struct NFTRestApi {}

impl NFTRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production NFT REST API URL.
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

        config.user_agent = build_user_agent("nft");
        if config.base_path.is_none() {
            config.base_path = Some(NFT_REST_API_PROD_URL.to_string());
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
        config.base_path = Some(NFT_REST_API_PROD_URL.to_string());
        NFTRestApi::from_config(config)
    }
}
