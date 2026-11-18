# Connection Mode Configuration

```rust
use binance_sdk::stock_options;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .mode(models::WebsocketMode::Pool(3)) // Use pool mode with a pool size of 3
    .build()?;

let client = stock_options::StockOptionsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = OrderReportV2StreamParams::builder("your-listen-key".to_string()).build()?;
let stream = connection.order_report_v2_stream(params).await?;
```
