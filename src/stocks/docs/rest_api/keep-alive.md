# Keep-Alive Configuration

```rust
use binance_sdk::stocks;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .keep_alive(false) // default is true
    .build()?;

let client = stocks::StocksRestApi::production(configuration);
let params = stocks::rest_api::ExchangeInfoParams::default();
let response = client.exchange_info(params).await?;
```
