# Key Pair Based Authentication

The WebSocket Streams connection URL is signed with your credentials. Instead of an `api_secret` (HMAC-SHA256), you can sign it with an `RSA` or `Ed25519` private key.

```rust
use binance_sdk::cms;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = cms::CmsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = cms::websocket_streams::ComAnnouncementEnParams::default();
let stream = connection.com_announcement_en(params).await?;
```
