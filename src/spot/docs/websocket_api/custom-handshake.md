# Custom WebSocket Handshake

The `handshake` option lets you supply your own async function that performs the
TLS/WebSocket handshake, so you can open a custom transport — such as a SOCKS5
tunnel — before the handshake runs. The SDK does not implement a proxy itself; it
simply calls your function for the initial connection, reconnects, and connection
renewal, applying the same 10-second timeout and error mapping as its built-in path.

The function receives the same inputs as
[`tokio_tungstenite::connect_async_tls_with_config`]: the HTTP upgrade `Request`,
an optional `WebSocketConfig`, a `disable_nagle` flag, and the `Option<Connector>`
resolved from the `agent` option (if set). The two options compose: `agent` only
customizes the TLS connector, while `handshake` controls how the transport is opened.
The example below simply forwards the arguments to the default connector; replace the
delegated call with your own transport setup to tunnel the connection.

```rust
use std::sync::Arc;

use tokio_tungstenite::connect_async_tls_with_config;

use binance_sdk::spot;
use binance_sdk::config::{self, WebsocketHandshakeFn};

let handshake: WebsocketHandshakeFn = Arc::new(|request, ws_config, disable_nagle, connector| {
    Box::pin(async move {
        // Establish any custom transport here (e.g. a SOCKS5 tunnel) before
        // completing the TLS and WebSocket handshake, then delegate to the
        // default connector to finish it.
        connect_async_tls_with_config(request, ws_config, disable_nagle, connector).await
    })
});

let configuration = config::ConfigurationWebsocketApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .handshake(handshake)
    .build()?;

let client = spot::SpotWsApi::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_api::ExchangeInfoParams::default();
let response = connection.exchange_info(params).await?;
```
