pub mod rest_api;

use crate::common::{
    config::ConfigurationRestApi, constants::FIAT_REST_API_PROD_URL, utils::build_user_agent,
};

/// Represents the Fiat REST API client for interacting with the Binance Fiat REST API.
///
/// This struct provides methods to create REST API clients for the production environment.
pub struct FiatRestApi {}

impl FiatRestApi {
    /// Creates a REST API client with the given configuration.
    ///
    /// If no base path is specified in the configuration, defaults to the production Fiat REST API URL.
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
        config.user_agent = build_user_agent("fiat");
        if config.base_path.is_none() {
            config.base_path = Some(FIAT_REST_API_PROD_URL.to_string());
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
        config.base_path = Some(FIAT_REST_API_PROD_URL.to_string());
        FiatRestApi::from_config(config)
    }
}
