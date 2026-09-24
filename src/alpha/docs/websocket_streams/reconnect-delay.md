# Reconnect Delay Configuration

```rust
use binance_sdk::alpha;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .reconnect_delay(3000) // Set reconnect delay to 3 seconds
    .build()?;

let client = alpha::AlphaWsStreams::production(configuration);
let connection = client.connect().await?;
let params = alpha::websocket_streams::AllBookTickerStreamParams::default();
let stream = connection.all_book_ticker_stream(params).await?;
```
