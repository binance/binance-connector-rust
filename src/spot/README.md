# Binance Rust SPOT Connector

[![Crates.io](https://img.shields.io/crates/v/binance-sdk)](https://crates.io/crates/binance-sdk)
[![docs.rs](https://img.shields.io/docsrs/binance-sdk)](https://docs.rs/binance-sdk)
[![Build Status](https://img.shields.io/github/actions/workflow/status/binance/binance-connector-rust/ci.yaml)](https://github.com/binance/binance-connector-rust/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This module provides the official Rust client for Binance's SPOT API, enabling developers to interact programmatically with Binance's SPOT trading platform. The library provides tools for retrieving market data, executing trades, and managing orders through three distinct endpoints:

- [REST API](./rest_api/mod.rs)
- [Websocket API](./websocket_api/mod.rs)
- [Websocket Stream](./websocket_streams/mod.rs)

## Table of Contents

- [Supported Features](#supported-features)
- [Installation](#installation)
- [Documentation](#documentation)
- [REST APIs](#rest-apis)
- [Websocket APIs](#websocket-apis)
- [Websocket Streams](#websocket-streams)
- [Testing](#testing)
- [Migration Guide](#migration-guide)
- [Contributing](#contributing)
- [License](#license)

## Supported Features

- REST API Endpoints:
  - `/api/*`
- WebSocket Endpoints: Real-time data streaming and request-response communication.
- Inclusion of test cases and examples for quick onboarding.

## Installation

Enable the `spot` feature in your `Cargo.toml` under `binance-sdk`:

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = ["spot"] }
```

In your code, import the `spot` client:

```rust
use binance_sdk::spot;
```

## Documentation

- **Crate documentation:** [docs.rs/binance_sdk](https://docs.rs/binance_sdk)
- **Official Binance SPOT API docs:** [Binance API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/README)

### REST APIs

All REST API endpoints are available through the [`rest_api`](./rest_api/mod.rs) module. Note that some endpoints require authentication using your Binance API credentials.

```rust
use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::spot;

let configuration = ConfigurationRestApi::builder()
  .api_key("YOUR_API_KEY")
  .api_secret("YOUR_SECRET_KEY")
  .build()?;

let client = spot::SpotRestApi::production(configuration);
let params = spot::rest_api::ExchangeInfoParams::default();
let response = client.exchange_info(params).await?;

let data = response.data().await?;
println!("{:#?}", data);
```

#### Configuration Options

The `spot` module can be configured with the following options via the `ConfigurationRestApi` builder pattern:

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
- `time_unit` (TimeUnit): Specify the time unit for timestamps (e.g., milliseconds or microseconds)

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

##### Time Unit

Specify the time unit for REST API timestamps (e.g., milliseconds or microseconds). See the [Time Unit example](./docs/rest_api/time-unit.md) for detailed usage.

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

#### Testnet

For testing purposes, the REST APIs also supports a testnet environment:

```rust
use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::spot;

let configuration = ConfigurationRestApi::builder()
  .api_key("YOUR_API_KEY")
  .api_secret("YOUR_SECRET_KEY")
  .build()?;

let client = spot::SpotRestApi::testnet(configuration);
```

### Websocket APIs

The WebSocket API provides request-response communication for market data and trading actions. Use the [`websocket_api`](./websocket_api/mod.rs) module to interact with these endpoints.

```rust
use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::spot;

let configuration = ConfigurationWebsocketApi::builder()
  .api_key("YOUR_API_KEY")
  .api_secret("YOUR_SECRET_KEY")
  .build()?;

let client = spot::SpotWsApi::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_api::ExchangeInfoParams::default();
let response = connection.exchange_info(params).await?;

let data = response.data().await?;
println!("{:#?}", data);
```

#### Configuration Options

The `spot` module can be configured with the following options via the `ConfigurationWebsocketApi` builder pattern:

- `timeout` (u64): WS request timeout in milliseconds (default: 5000)
- `reconnect_delay` (u64): Specify the delay between reconnection attempts in milliseconds (default: 5000)
- `mode` (WebsocketMode): Choose between `single` and `pool` connection modes
  - `Single`: A single WebSocket connection
  - `Pool`: A pool of WebSocket connections
- `agent` (AgentConnector): Customize the WebSocket agent for advanced configurations
- `private_key` (PrivateKey): RSA or ED25519 private key for request signing (raw string or PEM file path)
- `private_key_passphrase` (String): Passphrase for the private key, if encrypted
- `time_unit` (TimeUnit): Specify the time unit for timestamps (e.g., milliseconds or microseconds)

Refer to the [`configuration`](../common/config.rs) for more details.

##### Timeout

Set the timeout for WebSocket API requests in milliseconds. See the [Timeout example](./docs/websocket_api/timeout.md) for detailed usage.

##### Reconnect Delay

Specify the delay in milliseconds between WebSocket reconnection attempts. See the [Reconnect Delay example](./docs/websocket_api/reconnect-delay.md) for detailed usage.

##### WebSocket Agent

Customize the agent for advanced configurations. See the [WebSocket Agent example](./docs/websocket_api/agent.md) for detailed usage.

##### Connection Mode

Choose between `Single` and `Pool` connection modes for WebSocket connections. The `Single` mode uses a single WebSocket connection, while the `Pool` mode uses a pool of WebSocket connections. See the [Connection Mode example](./docs/websocket_api/connection-mode.md) for detailed usage.

##### Key Pair Based Authentication

Use RSA or ED25519 private keys for WebSocket API authentication. See the [Key Pair Authentication example](./docs/websocket_api/key-pair-authentication.md) for detailed usage.

##### Certificate Pinning

To enhance security, you can use certificate pinning with the `agent` option in the configuration. This ensures the client only communicates with servers using specific certificates. See the [Certificate Pinning example](./docs/websocket_api/certificate-pinning.md) for detailed usage.

##### Time Unit

Specify the time unit for WebSocket API timestamps (e.g., milliseconds or microseconds). See the [Time Unit example](./docs/websocket_api/time-unit.md) for detailed usage.

#### Subscribe to User Data Streams

You can consume the user data stream, which sends account-level events such as account and order updates. First do a `logon` to the websocket connection via WebSocket API; then:

```rust
use tracing::info;
use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::spot::{SpotWsApi, websocket_api::{SessionLogonParams, UserDataStreamSubscribeParams, UserDataStreamEventsResponse}};

let configuration = ConfigurationWebsocketApi::builder().build()?;

let client = SpotWsApi::production(configuration);
let connection = client.connect().await?;

let params = SessionLogonParams::default();

// Make the WS API call
let response = connection
  .session_logon(SessionLogonParams::default())
  .await?;

let (response, stream) = connection
  .user_data_stream_subscribe(UserDataStreamSubscribeParams::default())
  .await?;

stream.on_message(|data: UserDataStreamEventsResponse| {
  match data {
    UserDataStreamEventsResponse::OutboundAccountPosition(data) => {
      info!("outbound account position stream {:?}", data);
    }
    UserDataStreamEventsResponse::BalanceUpdate(data) => {
      info!("balance update stream {:?}", data);
    }
    UserDataStreamEventsResponse::Other(data) => {
      info!("unknown stream {:?}", data);
    }
    // …handle other variants…
  }
});
```

#### Testnet

For testing purposes, the Websocket API also supports a testnet environment:

```rust
use binance_sdk::config::ConfigurationWebsocketApi;
use binance_sdk::spot;

let configuration = ConfigurationWebsocketApi::builder()
  .api_key("YOUR_API_KEY")
  .api_secret("YOUR_SECRET_KEY")
  .build()?;

let client = spot::SpotWsApi::testnet(configuration);
```

### Websocket Streams

The WebSocket Streams provide real-time data feeds for market trades, candlesticks, and more. Use the [`websocket_streams`](./websocket_streams/mod.rs) module to interact with these endpoints.

```rust
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::spot;

let configuration = ConfigurationWebsocketStreams::builder().build()?;

let client = spot::SpotWsStreams::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_streams::AggTradeParams::default();
let stream = connection.agg_trade(params).await?;

stream.on_message(|data| {
  println!("{:#?}", data);
});
```

#### Configuration Options

The `spot` module can be configured with the following options via the `ConfigurationWebsocketStreams` builder pattern:

- `reconnect_delay` (u64): Specify the delay between reconnection attempts in milliseconds (default: 5000)
- `mode` (WebsocketMode): Choose between `single` and `pool` connection modes
  - `Single`: A single WebSocket connection
  - `Pool`: A pool of WebSocket connections
- `agent` (AgentConnector): Customize the WebSocket agent for advanced configurations
- `time_unit` (TimeUnit): Specify the time unit for timestamps (e.g., milliseconds or microseconds)

Refer to the [`configuration`](../common/config.rs) for more details.

##### Reconnect Delay

Specify the delay in milliseconds between WebSocket reconnection attempts. See the [Reconnect Delay example](./docs/websocket_streams/reconnect-delay.md) for detailed usage.

##### WebSocket Agent

Customize the agent for advanced configurations. See the [WebSocket Agent example](./docs/websocket_streams/agent.md) for detailed usage.

##### Connection Mode

Choose between `Single` and `Pool` connection modes for WebSocket connections. The `Single` mode uses a single WebSocket connection, while the `Pool` mode uses a pool of WebSocket connections. See the [Connection Mode example](./docs/websocket_streams/connection-mode.md) for detailed usage.

##### Certificate Pinning

To enhance security, you can use certificate pinning with the `agent` option in the configuration. This ensures the client only communicates with servers using specific certificates. See the [Certificate Pinning example](./docs/websocket_streams/certificate-pinning.md) for detailed usage.

##### Time Unit

Specify the time unit for WebSocket API timestamps (e.g., milliseconds or microseconds). See the [Time Unit example](./docs/websocket_streams/time-unit.md) for detailed usage.

#### Unsubscribing from Streams

You can unsubscribe from specific WebSocket streams using the `unsubscribe` method. This is useful for managing active subscriptions without closing the connection.

```rust
use tokio::time::{Duration, sleep};

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::spot;

let configuration = ConfigurationWebsocketStreams::builder().build()?;

let client = spot::SpotWsStreams::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_streams::AggTradeParams::default();
let stream = connection.agg_trade(params).await?;

stream.on_message(|data| {
  println!("{:#?}", data);
});

sleep(Duration::from_secs(10)).await;

stream.unsubscribe().await;
```

#### Testnet

For testing purposes, the Websocket Streams also supports a testnet environment:

```rust
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::spot;

let configuration = ConfigurationWebsocketStreams::builder().build()?;

let client = spot::SpotWsStreams::testnet(configuration);
```

### Automatic Connection Renewal

The WebSocket connection is automatically renewed for both WebSocket API and WebSocket Streams connections, before the 24 hours expiration of the API key. This ensures continuous connectivity.

## Testing

To run the tests for the SPOT module:

```bash
cargo test --features spot
```

Tests cover:

- REST API endpoint integration
- Error scenarios and edge cases

## Migration Guide

If you are upgrading from a legacy, single-crate connector that included SPOT support, see the [Migration Guide](../../MIGRATION.md) for instructions on enabling the `spot` feature.

## Contributing

Contributions are welcome!

Since this repository contains auto-generated code, we encourage you to start by opening a GitHub issue to discuss your ideas or suggest improvements. This helps ensure that changes align with the project's goals and auto-generation processes.

To contribute:

1. Open a GitHub issue describing your suggestion or the bug you've identified.
2. If it's determined that changes are necessary, the maintainers will merge the changes into the main branch.

Please ensure that all tests pass with `--features spot` if you're making a direct contribution. Submit a pull request only after discussing and confirming the change.

Thank you for your contributions!

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENCE) file for details.
