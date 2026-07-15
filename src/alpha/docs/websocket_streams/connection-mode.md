# Connection Mode Configuration

```rust
use binance_sdk::alpha;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .mode(models::WebsocketMode::Pool(3)) // Use pool mode with a pool size of 3
    .build()?;

let client = alpha::AlphaWsStreams::production(configuration);
let connection = client.connect().await?;
let params = alpha::websocket_streams::AllBookTickerStreamParams::default();
let stream = connection.all_book_ticker_stream(params).await?;
```
