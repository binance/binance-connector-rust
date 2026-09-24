# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::w3w_prediction;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = w3w_prediction::W3WPredictionRestApi::production(configuration);
let params = w3w_prediction::rest_api::ListPredictionCategoriesResponse::default();
let response = client.list_prediction_categories(params).await?;
```
