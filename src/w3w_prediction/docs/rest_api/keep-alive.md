# Keep-Alive Configuration

```rust
use binance_sdk::w3w_prediction;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .keep_alive(false) // default is true
    .build()?;

let client = w3w_prediction::W3WPredictionRestApi::production(configuration);
let params = w3w_prediction::rest_api::ListPredictionCategoriesResponse::default();
let response = client.list_prediction_categories(params).await?;
```
