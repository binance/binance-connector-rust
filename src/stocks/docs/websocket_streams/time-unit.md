# Time Unit Configuration

```rust
use binance_sdk::stocks;
use binance_sdk::config;
use binance_sdk::models;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .time_unit(models::TimeUnit::Microsecond) // Set time unit to microseconds
    .build()?;

let client = stocks::StocksWsStreams::production(configuration);
let connection = client.connect().await?;
let params = stocks::websocket_streams::CalendarStreamParams::default();
let stream = connection.calendar_stream(params).await?;
```
