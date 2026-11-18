# Retries Configuration

```rust
use binance_sdk::stock_options;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = stock_options::StockOptionsRestApi::production(configuration);
let params = OptionExpirationsParams::builder("NVDA".to_string()).build()?;
let response = client.option_expirations(params).await?;
```
