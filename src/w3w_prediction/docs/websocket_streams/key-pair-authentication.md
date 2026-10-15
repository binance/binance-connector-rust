# Key Pair Based Authentication

The WebSocket Streams connection URL is signed with your credentials. Instead of an `api_secret` (HMAC-SHA256), you can sign it with an `RSA` or `Ed25519` private key.

```rust
use binance_sdk::w3w_prediction;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = w3w_prediction::W3WPredictionWsStreams::production(configuration);
let connection = client.connect().await?;
let params = w3w_prediction::websocket_streams::PredictionOrderBookByMarketIdParams::builder("8859231".to_string()).build()?;
let stream = connection.prediction_order_book_by_market_id(params).await?;
```
