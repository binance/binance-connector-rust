# **Migration Guide: Transition from Legacy Singleton Rust Connector**

With the move towards a feature-flag–driven, all-in-one crate, Binance Rust connectors are now bundled under `binance-sdk`. This guide explains how to migrate from the legacy singleton crate (which supported only a subset of Binance products and did not use feature flags) to the new, unified, modular crate.

## Overview of Changes

| Feature               | Legacy Singleton Crate                    | New Unified Crate                                         |
| --------------------- | ----------------------------------------- | --------------------------------------------------------- |
| Crate Name            | `binance_spot_connector_rust` (legacy)    | `binance-sdk` (new)                                 |
| API Coverage          | Only a few products                       | All publicly available Binance APIs, enabled via features |
| Configuration         | Hard-coded product support                | Feature flags enable specific API modules                 |
| Imports               | `use binance_spot_connector_rust;`        | `use binance_sdk::spot;`                            |
| Dependencies          | Single dependency entry                   | Single dependency entry with features                     |

## Migration Steps

### Step 1: Remove Legacy Crate Dependency

Open your `Cargo.toml` and remove the outdated entry. For example, if you had:

```toml
[dependencies]
binance_spot_connector_rust = "1.3.0"  # legacy single-crate
```

Remove this line completely.

### Step 2: Add New `binance-sdk` with Feature Flags

Replace the legacy dependency with the new unified crate, specifying feature flags for only the APIs you need. For example, to include `Spot` and `USDS-M Futures` support:

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = ["derivatives_trading_usds_futures", "spot"] }
```

If you previously consumed multiple products through the legacy crate, enable all corresponding features:

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = [
  "spot",
  "derivatives_trading_usds_futures",
  "margin-trading",
] }
```

### Step 3: Update `use` Statements

Legacy code imported directly from `binance_spot_connector_rust`. For example:

```rust
use binance_spot_connector_rust::market;
```

Update these to reference the new submodule paths. For example:

```rust
use binance_sdk::spot;
```

### Step 4: Initialize Clients Under the Unified Crate

Client initialization remains the same in terms of function calls but now relies on the enabled features. For example:

```rust
// Legacy
let http_client = BinanceHttpClient::with_url("https://api.binance.com");
let request = market::exchange_info().symbol("BNBUSDT");
let response = http_client.send(request).await?.into_body_str().await?;

// New (Spot)
let rest_conf = ConfigurationRestApi::builder().api_key("your-key").api_secret("your-secret").build()?;
let rest_client = spot::SpotRestApi::production(rest_conf);
let params = ExchangeInfoParams::builder().symbol("BNBUSDT".to_string()).build()?;
let response = rest_client.exchange_info(params)
```

Compilation errors will reveal any remaining outdated paths.

### Step 5: Verify Endpoint and Model Consistency

Since all connectors are auto-generated from the same OpenAPI definitions, function signatures and model types remain consistent. If you have custom code that assumed legacy-only behavior, review it against the new docs at [docs.rs/binance-sdk](https://docs.rs/binance-sdk).

## Backward Compatibility

* If a specific product feature is not yet available under the new crate, you may temporarily continue using the legacy singleton crate until the corresponding feature is released. Legacy crates will be maintained for a brief deprecation period.
* Future updates and new product support will only be added to the new unified crate behind feature flags.

---

## FAQs

### What if my project used a legacy single crate for multiple products?

Simply enable all the corresponding features under the new dependency. For instance:

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = ["spot", "margin-trading"] }
```

Remove the old `binance_spot_connector_rust = "1.3.0"` entry.

### How do I find available features and their names?

See the [Available Modules](./README.md#available-modules) section in the main README or check the crate’s `Cargo.toml` feature list directly.

### Will the unified crate introduce breaking changes?

We aim to maintain stability for existing features. Any breaking changes will be documented in release notes and communicated prior to release.

---

For further assistance, open an issue in the repository or refer to the official [Binance API Documentation](https://developers.binance.com).
