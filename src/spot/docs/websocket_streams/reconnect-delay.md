# Reconnect Delay Configuration

```rust
use binance_sdk::spot;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .reconnect_delay(3000) // Set reconnect delay to 3 seconds
    .build()?;

let client = spot::SpotWsStreams::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_streams::AggTradeParams::default();
let stream = connection.agg_trade(params).await?;
```
