# Binance Rust CMS Connector

[![Crates.io](https://img.shields.io/crates/v/binance-sdk)](https://crates.io/crates/binance-sdk)
[![docs.rs](https://img.shields.io/docsrs/binance-sdk)](https://docs.rs/binance-sdk)
[![Build Status](https://img.shields.io/github/actions/workflow/status/binance/binance-connector-rust/ci.yaml)](https://github.com/binance/binance-connector-rust/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This module provides the official Rust client for Binance's CMS (Content Management System) API, enabling developers to receive Binance announcements programmatically in real time through the WebSocket Streams:

- [WebSocket Streams](./websocket_streams/mod.rs)

## Table of Contents

- [Supported Features](#supported-features)
- [Installation](#installation)
- [Documentation](#documentation)
- [Websocket Streams](#websocket-streams)
- [Logging](#logging)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Supported Features

- WebSocket Endpoints: Real-time data streaming.
- Inclusion of test cases and examples for quick onboarding.

## Installation

Enable the `cms` feature in your `Cargo.toml` under `binance-sdk`:

```toml
[dependencies]
binance-sdk = { version = "75.0.0", features = ["cms"] }
```

In your code, import the `cms` client:

```rust
use binance_sdk::cms;
```

## Documentation

- **Crate documentation:** [docs.rs/binance_sdk](https://docs.rs/binance_sdk)
- **Official Binance CMS API docs:** [Binance API Documentation](https://developers.binance.com/en/docs/products/announcements/announcement)

### Websocket Streams

The WebSocket Streams provide real-time push notifications for Binance announcements. Use the [`websocket_streams`](./websocket_streams/mod.rs) module to interact with these endpoints.

These streams are signed, so an API key and secret (or a private key) are required.

```rust
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::cms;

let configuration = ConfigurationWebsocketStreams::builder()
  .api_key("YOUR_API_KEY")
  .api_secret("YOUR_SECRET_KEY")
  .build()?;

let client = cms::CmsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = cms::websocket_streams::ComAnnouncementEnParams::default();
let stream = connection.com_announcement_en(params).await?;

stream.on_message(|data| {
  println!("{:#?}", data);
});
```

#### Configuration Options

The `cms` module can be configured with the following options via the `ConfigurationWebsocketStreams` builder pattern:

- `api_key` (String): Your Binance API key (required). It is sent in the `X-MBX-APIKEY` header on connection
- `api_secret` (String): Your Binance API secret, used to sign the connection URL with HMAC-SHA256
- `private_key` (PrivateKey): RSA or ED25519 private key, used instead of `api_secret` to sign the connection URL (raw string or PEM file path)
- `private_key_passphrase` (String): Passphrase for the private key, if encrypted
- `reconnect_delay` (u64): Specify the delay between reconnection attempts in milliseconds (default: 5000)
- `mode` (WebsocketMode): Choose between `single` and `pool` connection modes
  - `Single`: A single WebSocket connection
  - `Pool`: A pool of WebSocket connections
- `agent` (AgentConnector): Customize the WebSocket agent for advanced configurations
- `handshake` (WebsocketHandshakeFn): Provide a custom async TLS/WebSocket handshake (e.g. to tunnel through a SOCKS5 proxy)

Refer to the [`configuration`](../common/config.rs) for more details.

##### Key Pair Based Authentication

Sign the connection URL with an `RSA` or `Ed25519` private key instead of an API secret. See the [Key Pair Based Authentication example](./docs/websocket_streams/key-pair-authentication.md) for detailed usage.

##### Reconnect Delay

Specify the delay in milliseconds between WebSocket reconnection attempts. See the [Reconnect Delay example](./docs/websocket_streams/reconnect-delay.md) for detailed usage.

##### WebSocket Agent

Customize the agent for advanced configurations. See the [WebSocket Agent example](./docs/websocket_streams/agent.md) for detailed usage.

##### Custom Handshake

Supply a custom async TLS/WebSocket handshake to open your own transport (such as a SOCKS5 tunnel) before the handshake completes. See the [Custom Handshake example](./docs/websocket_streams/custom-handshake.md) for detailed usage.

##### Connection Mode

Choose between `Single` and `Pool` connection modes for WebSocket connections. The `Single` mode uses a single WebSocket connection, while the `Pool` mode uses a pool of WebSocket connections. See the [Connection Mode example](./docs/websocket_streams/connection-mode.md) for detailed usage.

##### Certificate Pinning

To enhance security, you can use certificate pinning with the `agent` option in the configuration. This ensures the client only communicates with servers using specific certificates. See the [Certificate Pinning example](./docs/websocket_streams/certificate-pinning.md) for detailed usage.

#### Unsubscribing from Streams

You can unsubscribe from specific WebSocket streams using the `unsubscribe` method. This is useful for managing active subscriptions without closing the connection.

```rust
use tokio::time::{Duration, sleep};

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::cms;

let configuration = ConfigurationWebsocketStreams::builder()
  .api_key("YOUR_API_KEY")
  .api_secret("YOUR_SECRET_KEY")
  .build()?;

let client = cms::CmsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = cms::websocket_streams::ComAnnouncementEnParams::default();
let stream = connection.com_announcement_en(params).await?;

stream.on_message(|data| {
  println!("{:#?}", data);
});

sleep(Duration::from_secs(10)).await;

stream.unsubscribe().await;
```

### Automatic Connection Renewal

The WebSocket connection is automatically renewed for WebSocket Streams connections, before the 24 hours expiration of the API key. This ensures continuous connectivity. Active streams are re-subscribed automatically after every reconnection or renewal.

## Logging

This crate ships with an optional default logger that you can enable with a single call:

```rust
use binance_sdk::logger;

fn main() {
    // Initialize the default logger once at the start of your application
    logger::init();

    // ... rest of your code
}
```

The logger integrates with the Rust `tracing` ecosystem and behaves as follows:

- If **another global subscriber is already set**, `logger::init()` is a no-op and does not override your existing logging setup.
- If **no subscriber is set yet**, it installs the crate’s default global subscriber (with sensible defaults) so you immediately get structured logs from the connector.

## Testing

To run the tests for the CMS module:

```bash
cargo test --features cms
```

Tests cover:

- WebSocket Streams

## Contributing

Contributions are welcome!

Since this repository contains auto-generated code, we encourage you to start by opening a GitHub issue to discuss your ideas or suggest improvements. This helps ensure that changes align with the project's goals and auto-generation processes.

To contribute:

1. Open a GitHub issue describing your suggestion or the bug you've identified.
2. If it's determined that changes are necessary, the maintainers will merge the changes into the main branch.

Please ensure that all tests pass with `--features cms` if you're making a direct contribution. Submit a pull request only after discussing and confirming the change.

Thank you for your contributions!

## Disclaimer

This SDK is provided by Binance on an "as is" and "as available" basis for use at your own risk. Binance makes no representations or warranties of any kind, whether express or implied, as to the operation of the SDK, its accuracy, reliability, completeness, or fitness for any particular purpose.

To the fullest extent permitted by law, Binance shall not be liable for any losses, damages, or expenses of any kind arising from or in connection with your use of, or inability to use, this SDK, including but not limited to any financial losses resulting from errors, bugs, interruptions, or inaccuracies in the SDK.

Your use of this SDK to access the Binance Platform is subject to the Binance API Key Terms and the Binance Terms of Use, which shall prevail in the event of any conflict with this disclaimer. You are solely responsible for any orders or transactions executed through the Binance Platform using this SDK.

This SDK is not intended to constitute investment advice or a recommendation to buy, sell, or hold any digital asset. You should independently evaluate and verify all information before acting.

- [Binance Terms of Use](https://www.binance.com/en/terms)
- [Binance API Key Terms](https://www.binance.com/en/about-legal/terms-binance-api)

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENCE) file for details.
