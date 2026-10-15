# Connection Mode Configuration

```rust
use binance_sdk::cms;
use binance_sdk::{config, models};

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .mode(models::WebsocketMode::Pool(3)) // Use pool mode with a pool size of 3
    .build()?;

let client = cms::CmsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = cms::websocket_streams::ComAnnouncementEnParams::default();
let stream = connection.com_announcement_en(params).await?;
```
