# Connection Mode Configuration

```rust
use binance_sdk::derivatives_trading_options;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .mode(models::WebsocketMode::Pool(3)) // Use pool mode with a pool size of 3
    .build()?;

let client = derivatives_trading_options::DerivativesTradingOptionsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = derivatives_trading_options::websocket_streams::NewSymbolInfoParams::default();
let stream = connection.new_symbol_info(params).await?;
```
