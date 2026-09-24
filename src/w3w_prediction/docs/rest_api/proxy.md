# Proxy Configuration

```rust
use binance_sdk::w3w_prediction;
use binance_sdk::config;

let proxy_config = config::ProxyConfig {
    host: "127.0.0.1".to_string(),
    port: 8080,
    protocol: Some("http".to_string()),
    auth: Some(config::ProxyAuth {
        username: "proxy-user".to_string(),
        password: "proxy-password".to_string(),
    }),
};

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .proxy(proxy_config)
    .build()?;

let client = w3w_prediction::W3WPredictionRestApi::production(configuration);
let params = w3w_prediction::rest_api::ListPredictionCategoriesResponse::default();
let response = client.list_prediction_categories(params).await?;
```
