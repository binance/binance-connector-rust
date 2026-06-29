# Key Pair Based Authentication

```rust
use binance_sdk::w3w_prediction;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = w3w_prediction::W3WPredictionRestApi::production(configuration);
let params = w3w_prediction::rest_api::ListPredictionCategoriesResponse::default();
let response = client.list_prediction_categories(params).await?;
```
