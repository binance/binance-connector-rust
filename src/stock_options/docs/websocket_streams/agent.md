# WebSocket Agent Configuration

```rust
use tokio_tungstenite::Connector;
use native_tls::{TlsConnector, Protocol};

use binance_sdk::stock_options;
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

let client = stock_options::StockOptionsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = OrderReportV2StreamParams::builder("your-listen-key".to_string()).build()?;
let stream = connection.order_report_v2_stream(params).await?;
```
