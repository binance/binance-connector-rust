# Retries Configuration

```rust
use binance_sdk::stocks;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = stocks::StocksRestApi::production(configuration);
let params = stocks::rest_api::ExchangeInfoParams::default();
let response = client.exchange_info(params).await?;
```
