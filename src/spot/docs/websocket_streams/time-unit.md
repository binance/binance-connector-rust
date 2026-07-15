# Time Unit Configuration

```rust
use binance_sdk::spot;
use binance_sdk::config;
use binance_sdk::models;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .time_unit(models::TimeUnit::Microsecond) // Set time unit to microseconds
    .build()?;

let client = spot::SpotWsStreams::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_streams::AggTradeParams::default();
let stream = connection.agg_trade(params).await?;
```
