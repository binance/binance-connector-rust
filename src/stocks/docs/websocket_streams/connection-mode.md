# Connection Mode Configuration

```rust
use binance_sdk::stocks;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .mode(models::WebsocketMode::Pool(3)) // Use pool mode with a pool size of 3
    .build()?;

let client = stocks::StocksWsStreams::production(configuration);
let connection = client.connect().await?;
let params = stocks::websocket_streams::CalendarStreamParams::default();
let stream = connection.calendar_stream(params).await?;
```
