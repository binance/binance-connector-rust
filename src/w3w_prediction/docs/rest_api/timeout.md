# Timeout

```rust
use binance_sdk::w3w_prediction;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(5000)
    .build()?;

let client = w3w_prediction::W3WPredictionRestApi::production(configuration);
let response = client.list_prediction_categories().await?;
```
