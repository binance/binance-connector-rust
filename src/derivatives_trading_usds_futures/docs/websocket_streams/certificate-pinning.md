# WebSocket Agent Configuration

```rust
use std::fs;
use tokio_tungstenite::Connector;
use native_tls::{Certificate, TlsConnector, Protocol};

use binance_sdk::derivatives_trading_usds_futures;
use binance_sdk::config;

let cert_pem = fs::read("/path/to/pinned_cert.pem")?;
let cert = Certificate::from_pem(&cert_pem)?;

let native_tls = TlsConnector::builder()
    .add_root_certificate(cert)
    .build()?;
let ws_connector = Connector::NativeTls(native_tls);

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(config::AgentConnector(ws_connector))
    .build()?;

let client = derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesWsStreams::production(configuration);
let connection = client.connect().await?;
let params = derivatives_trading_usds_futures::websocket_streams::AllBookTickersStreamParams::default();
let stream = connection.all_book_tickers_stream(params).await?;
```
