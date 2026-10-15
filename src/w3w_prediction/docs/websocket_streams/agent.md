# WebSocket Agent Configuration

```rust
use tokio_tungstenite::Connector;
use native_tls::{TlsConnector, Protocol};

use binance_sdk::w3w_prediction;
use binance_sdk::config;

let native_tls = TlsConnector::builder()
    .min_protocol_version(Some(Protocol::Tlsv12))
    .build()?;
let ws_connector = Connector::NativeTls(native_tls);

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(config::AgentConnector(ws_connector))
    .build()?;

let client = w3w_prediction::W3WPredictionWsStreams::production(configuration);
let connection = client.connect().await?;
let params = w3w_prediction::websocket_streams::PredictionOrderBookByMarketIdParams::builder("8859231".to_string()).build()?;
let stream = connection.prediction_order_book_by_market_id(params).await?;
```
