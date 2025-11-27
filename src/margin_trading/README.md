# Binance Rust Margin Trading Connector

[![Crates.io](https://img.shields.io/crates/v/binance-sdk)](https://crates.io/crates/binance-sdk)
[![docs.rs](https://img.shields.io/docsrs/binance-sdk)](https://docs.rs/binance-sdk)
[![Build Status](https://img.shields.io/github/actions/workflow/status/binance/binance-connector-rust/ci.yaml)](https://github.com/binance/binance-connector-rust/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This module provides the official Rust client for Binance's Margin Trading API, enabling developers to interact programmatically with Binance's Margin Trading trading platform. The library provides tools to use funds provided by a third party to conduct asset transactions through the REST API:

- [REST API](./rest_api/mod.rs)
- [Websocket Stream](./websocket_streams/mod.rs)

## Table of Contents

- [Supported Features](#supported-features)
- [Installation](#installation)
- [Documentation](#documentation)
- [REST APIs](#rest-apis)
- [Websocket Streams](#websocket-streams)
- [Logging](#logging)
- [Testing](#testing)
- [Migration Guide](#migration-guide)
- [Contributing](#contributing)
- [License](#license)

## Supported Features

- REST API Endpoints:
  - `/sapi/v1/margin/*`
  - `/sapi/v1/bnbBurn/*`
  - `/sapi/v1/userDataStream/*`
- Inclusion of test cases and examples for quick onboarding.

## Installation

Enable the `margin_trading` feature in your `Cargo.toml` under `binance-sdk`:

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = ["margin_trading"] }
```

In your code, import the `margin_trading` client:

```rust
use binance_sdk::margin_trading;
```

## Documentation

- **Crate documentation:** [docs.rs/binance_sdk](https://docs.rs/binance_sdk)
- **Official Binance Margin Trading API docs:** [Binance API Documentation](https://developers.binance.com/docs/margin_trading)

### REST APIs

All REST API endpoints are available through the [`rest_api`](./rest_api/mod.rs) module. Note that some endpoints require authentication using your Binance API credentials.

```rust
use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::margin_trading;

let configuration = ConfigurationRestApi::builder()
  .api_key("YOUR_API_KEY")
  .api_secret("YOUR_SECRET_KEY")
  .build()?;

let client = margin_trading::MarginTradingRestApi::production(configuration);
let params = margin_trading::rest_api::GetSummaryOfMarginAccountParams::default();
let response = client.get_summary_of_margin_account(params).await?;

let data = response.data().await?;
println!("{:#?}", data);
```

#### Configuration Options

The `margin_trading` module can be configured with the following options via the `ConfigurationRestApi` builder pattern:

- `timeout` (u64): Request timeout in milliseconds (default: 1000)
- `proxy` (ProxyConfig): HTTP/HTTPS proxy settings
  - `host` (String): Proxy server hostname.
  - `port` (u16): Proxy server port.
  - `protocol` (String): Proxy protocol (http or https).
  - `auth` (ProxyAuth): Proxy authentication credentials:
    - `username` (String): Proxy username.
    - `password` (String): Proxy password.
- `keep_alive` (bool): Enable HTTP keep-alive (default: true)
- `compression` (bool): Enable response compression (default: true)
- `retries` (u32): Number of retry attempts for failed requests (default: 3)
- `backoff` (u64): Delay in milliseconds between retries (default: 1000)
- `agent` (HttpAgent): Custom HTTP agent for advanced TLS configuration
- `private_key` (PrivateKey): RSA or ED25519 private key for request signing (raw string or PEM file path)
- `private_key_passphrase` (String): Passphrase for the private key, if encrypted

Refer to the [`configuration`](../common/config.rs) for more details.

##### Timeout

You can configure a timeout for requests in milliseconds. If the request exceeds the specified timeout, it will be aborted. See the [Timeout example](./docs/rest_api/timeout.md) for detailed usage.

##### Proxy

The REST API supports HTTP/HTTPS proxy configurations. See the [Proxy example](./docs/rest_api/proxy.md) for detailed usage.

##### Keep-Alive

Enable HTTP keep-alive for persistent connections. See the [Keep-Alive example](./docs/rest_api/keep-alive.md) for detailed usage.

##### Compression

Enable or disable response compression. See the [Compression example](./docs/rest_api/compression.md) for detailed usage.

##### Retries

Configure the number of retry attempts and delay in milliseconds between retries for failed requests. See the [Retries example](./docs/rest_api/retries.md) for detailed usage.

##### HTTPS Agent

Customize the HTTPS agent for advanced TLS configurations. See the [HTTPS Agent example](./docs/rest_api/https-agent.md) for detailed usage.

##### Key Pair Based Authentication

The REST API supports key pair-based authentication for secure communication. You can use `RSA` or `Ed25519` keys for signing requests. See the [Key Pair Based Authentication example](./docs/rest_api/key-pair-authentication.md) for detailed usage.

##### Certificate Pinning

To enhance security, you can use certificate pinning with the `agent` option in the configuration. This ensures the client only communicates with servers using specific certificates. See the [Certificate Pinning example](./docs/rest_api/certificate-pinning.md) for detailed usage.

#### Error Handling

Errors are represented by the following types:

- `ConnectorClientError`: General client error
- `UnauthorizedError`: Invalid or missing authentication
- `ForbiddenError`: Access to resource forbidden
- `TooManyRequestsError`: Rate limit exceeded
- `RateLimitBanError`: IP banned due to rate limits
- `ServerError`: Internal server error
- `NetworkError`: Network connectivity issues
- `NotFoundError`: Resource not found
- `BadRequestError`: Invalid request parameters

See the [Error Handling example](./docs/rest_api/error-handling.md) for detailed usage. Refer to the [`error`](../common/errors.rs) module for more information.

### Websocket Streams

The WebSocket Streams for `margin_trading` is used for subscribing to risk and trade data streams. Use the [`websocket_streams`](./websocket_streams/mod.rs) module to interact with it.

#### Configuration Options

The `margin_trading` module can be configured with the following options via the `ConfigurationWebsocketStreams` builder pattern:

- `reconnect_delay` (u64): Specify the delay between reconnection attempts in milliseconds (default: 5000)
- `mode` (WebsocketMode): Choose between `single` and `pool` connection modes
  - `Single`: A single WebSocket connection
  - `Pool`: A pool of WebSocket connections
- `agent` (AgentConnector): Customize the WebSocket agent for advanced configurations

Refer to the [`configuration`](../common/config.rs) for more details.

#### Subscribe to Risk and Trade Data Streams

You can consume the risk and trade data streams, which sends account-level events such as account and order updates. First create a listen-key via REST API; then:

```rust
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::margin_trading::{MarginTradingWsStreams, websocket_streams::{RiskDataStreamEventsResponse, TradeDataStreamEventsResponse}};

let configuration = ConfigurationWebsocketStreams::builder().build()?;

let client = MarginTradingWsStreams::production(configuration);
let connection = client.connect().await?;
let risk_stream = connection.risk_data("listen_key".to_string(), Some("custom_id".to_string())).await?;

risk_stream.on_message(|data: RiskDataStreamEventsResponse| {
  match data {
    RiskDataStreamEventsResponse::UserLiabilityChange(data) => {
      info!("user liability change stream {:?}", data);
    }
    RiskDataStreamEventsResponse::MarginLevelStatusChange(data) => {
      info!("margin level status change stream {:?}", data);
    }
    RiskDataStreamEventsResponse::Other(data) => {
      info!("unknown stream {:?}", data);
    }
  }
});

let trade_stream = connection.trade_data("listen_key".to_string(), Some("custom_id".to_string())).await?;

trade_stream.on_message(|data: TradeDataStreamEventsResponse| {
  match data {
    TradeDataStreamEventsResponse::OutboundAccountPosition(data) => {
      info!("outbound account position stream {:?}", data);
    }
    TradeDataStreamEventsResponse::BalanceUpdate(data) => {
      info!("balance update stream {:?}", data);
    }
    TradeDataStreamEventsResponse::Other(data) => {
      info!("unknown stream {:?}", data);
    }
    // …handle other variants…
  }
});
```

#### Unsubscribing from Risk and Trade Data Streams

You can unsubscribe from the risk and trade data streams using the `unsubscribe` method. This is useful for managing active subscriptions without closing the connection.

```rust
use tokio::time::{Duration, sleep};

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::margin_trading::{MarginTradingWsStreams, websocket_streams::{RiskDataStreamEventsResponse, TradeDataStreamEventsResponse}};

let configuration = ConfigurationWebsocketStreams::builder().build()?;

let client = MarginTradingWsStreams::production(configuration);
let connection = client.connect().await?;
let risk_stream = connection.risk_data("listen_key".to_string(), Some("custom_id".to_string())).await?;

trade_stream.on_message(|data: TradeDataStreamEventsResponse| {
  match data {
    TradeDataStreamEventsResponse::OutboundAccountPosition(data) => {
      info!("outbound account position stream {:?}", data);
    }
    TradeDataStreamEventsResponse::BalanceUpdate(data) => {
      info!("balance update stream {:?}", data);
    }
    TradeDataStreamEventsResponse::Other(data) => {
      info!("unknown stream {:?}", data);
    }
    // …handle other variants…
  }
});

sleep(Duration::from_secs(10)).await;

trade_stream.unsubscribe().await;
```

### Automatic Connection Renewal

The WebSocket connection is automatically renewed for WebSocket Streams connections, before the 24 hours expiration of the API key. This ensures continuous connectivity.

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

To run the tests for the Margin Trading module:

```bash
cargo test --features margin_trading
```

Tests cover:

- REST API endpoint integration
- Error scenarios and edge cases

## Migration Guide

If you are upgrading from a legacy, single-crate connector that included Margin Trading support, see the [Migration Guide](../../MIGRATION.md) for instructions on enabling the `margin_trading` feature.

## Contributing

Contributions are welcome!

Since this repository contains auto-generated code, we encourage you to start by opening a GitHub issue to discuss your ideas or suggest improvements. This helps ensure that changes align with the project's goals and auto-generation processes.

To contribute:

1. Open a GitHub issue describing your suggestion or the bug you've identified.
2. If it's determined that changes are necessary, the maintainers will merge the changes into the main branch.

Please ensure that all tests pass with `--features margin_trading` if you're making a direct contribution. Submit a pull request only after discussing and confirming the change.

Thank you for your contributions!

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENCE) file for details.
