# Timeout

```rust
use binance_sdk::alpha;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(5000)
    .build()?;

let client = alpha::AlphaRestApi::production(configuration);
let response = client.get_exchange_info(params).await?;
```
