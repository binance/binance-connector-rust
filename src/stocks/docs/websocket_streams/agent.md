# WebSocket Agent Configuration

```rust
use tokio_tungstenite::Connector;
use native_tls::{TlsConnector, Protocol};

use binance_sdk::stocks;
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

let client = stocks::StocksWsStreams::production(configuration);
let connection = client.connect().await?;
let params = stocks::websocket_streams::CalendarStreamParams::default();
let stream = connection.calendar_stream(params).await?;
```
