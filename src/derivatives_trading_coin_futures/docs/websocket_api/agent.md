# WebSocket Agent Configuration

```rust
use tokio_tungstenite::Connector;
use native_tls::{TlsConnector, Protocol};

use binance_sdk::derivatives_trading_coin_futures;
use binance_sdk::config;

let native_tls = TlsConnector::builder()
    .min_protocol_version(Some(Protocol::Tlsv12))
    .build()?;
let ws_connector = Connector::NativeTls(native_tls);

let configuration = config::ConfigurationWebsocketApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(config::AgentConnector(ws_connector))
    .build()?;

let client = derivatives_trading_coin_futures::DerivativesTradingCoinFuturesWsApi::production(configuration);
let connection = client.connect().await?;
let params = derivatives_trading_coin_futures::websocket_api::AccountInformationParams::default();
let response = connection.account_information(params).await?;
```
