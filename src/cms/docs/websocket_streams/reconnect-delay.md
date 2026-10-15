# Reconnect Delay Configuration

```rust
use binance_sdk::cms;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .reconnect_delay(3000) // Set reconnect delay to 3 seconds
    .build()?;

let client = cms::CmsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = cms::websocket_streams::ComAnnouncementEnParams::default();
let stream = connection.com_announcement_en(params).await?;
```
