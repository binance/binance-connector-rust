# Reconnect Delay Configuration

```rust
use binance_sdk::w3w_prediction;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .reconnect_delay(3000) // Set reconnect delay to 3 seconds
    .build()?;

let client = w3w_prediction::W3WPredictionWsStreams::production(configuration);
let connection = client.connect().await?;
let params = w3w_prediction::websocket_streams::PredictionOrderBookByMarketIdParams::builder("8859231".to_string()).build()?;
let stream = connection.prediction_order_book_by_market_id(params).await?;
```
