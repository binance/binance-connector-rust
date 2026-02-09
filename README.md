# Binance Rust Connector

[![Build Status](https://img.shields.io/github/actions/workflow/status/binance/binance-connector-rust/ci.yaml)](https://github.com/binance/binance-connector-rust/actions)
[![Open Issues](https://img.shields.io/github/issues/binance/binance-connector-rust)](https://github.com/binance/binance-connector-rust/issues)
[![Crates.io](https://img.shields.io/crates/v/binance-sdk)](https://crates.io/crates/binance-sdk)
[![docs.rs](https://img.shields.io/docsrs/binance-sdk)](https://docs.rs/binance-sdk)
[![Known Vulnerabilities](https://snyk.io/test/github/binance/binance-connector-rust/badge.svg)](https://snyk.io/test/github/binance/binance-connector-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official collection of auto-generated Rust SDK modules for Binance APIs.

## Migration Guide

If you're upgrading from the previous connector, refer to our [Migration Guide](./MIGRATION.md) for detailed steps on transitioning to the new modular structure. The legacy connector will still be available for a limited time. You can find the code for the old connector in the `legacy` branch.

## Prerequisites

Before using the connector, ensure you have:

* **Rust** (version 1.86.0 or later)
* **cargo** (comes with Rust)

Install or update Rust via [rustup](https://rustup.rs/):

```bash
rustup install 1.86.0
rustup default 1.86.0
```

## Available Modules

All connectors are bundled within the single `binance-sdk` crate. Enable only the modules you need by specifying feature flags. Available features are:

* [`algo`](./src/algo) – Algo Trading connector
* [`alpha`](./src/alpha) – Alpha connector
* [`c2c`](./src/c2c) – C2C connector
* [`convert`](./src/convert) – Convert connector
* [`copy_trading`](./src/copy_trading) – Copy Trading connector
* [`crypto_loan`](./src/crypto_loan) – Crypto Loan connector
* [`derivatives_trading_coin_futures`](./src/futures_coin) – Derivatives Trading (COIN-M Futures) connector
* [`derivatives_trading_options`](./src/futures_options) – Derivatives Trading (Options) connector
* [`derivatives_trading_portfolio_margin`](./src/portfolio_margin) – Derivatives Trading (Portfolio Margin) connector
* [`derivatives_trading_portfolio_margin_pro`](./src/portfolio_margin_pro) – Derivatives Trading (Portfolio Margin Pro) connector
* [`derivatives_trading_usds_futures`](./src/futures_usds) – Derivatives Trading (USDS-M Futures) connector
* [`dual_investment`](./src/dual_investment) – Dual Investment connector
* [`fiat`](./src/fiat) – Fiat connector
* [`gift_card`](./src/gift_card) – Gift Card connector
* [`margin_trading`](./src/margin_trading) – Margin Trading connector
* [`mining`](./src/mining) – Mining connector
* [`nft`](./src/nft) – NFT connector
* [`pay`](./src/pay) – Pay connector
* [`rebate`](./src/rebate) – Rebate connector
* [`simple_earn`](./src/simple_earn) – Simple Earn connector
* [`spot`](./src/spot) – Spot Trading connector
* [`staking`](./src/staking) – Staking connector
* [`sub_account`](./src/sub_account) – Sub Account connector
* [`vip_loan`](./src/vip_loan) – VIP Loan connector
* [`wallet`](./src/wallet) – Wallet connector

## Documentation

* **Crate documentation:** [docs.rs/binance_sdk](https://docs.rs/binance_sdk)
* **Official Binance API docs:** [Binance API Documentation](https://developers.binance.com)

## Installation

Add `binance-sdk` to your `Cargo.toml`, enabling only the features you need. For example, to include `Spot` and `USDS-M Futures` modules:

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = ["derivatives_trading_usds_futures", "spot"] }
```

If you require all available connectors:

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = ["all"] }
```

## TLS Backend Selection
This library supports both **OpenSSL** (default) and **Rustls** backends.

**Default (OpenSSL):** Standard installation uses OpenSSL. This requires OpenSSL development headers to be installed on your system.

```toml
[dependencies]
binance-sdk = { version = "1.0.0", features = ["spot"] }
```

Using Rustls (Pure Rust): To use rustls (useful for cross-compilation or avoiding C-dependencies), you must disable default features and enable rustls-tls.

Note: Private key signing features currently require the openssl-tls feature.

```toml
[dependencies]
binance-sdk = { version = "1.0.0", default-features = false, features = ["rustls-tls", "spot"] }
```

## Contributing

This repository contains auto-generated code using OpenAPI Generator. To contribute or request changes:

1. **Open a GitHub issue** to discuss new features, bugs, or enhancements.
2. **Fork the repository**, make your changes, and submit a pull request.
3. **Respect the code generation workflow** — manual edits to generated files will be overwritten.

Please ensure all new code is covered by existing or new tests. We follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for naming and documentation.

## License

This project is licensed under the MIT License. See the [LICENCE](./LICENCE) file for details.
