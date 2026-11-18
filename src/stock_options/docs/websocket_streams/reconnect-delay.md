# Reconnect Delay Configuration

```rust
use binance_sdk::stock_options;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketStreams::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .reconnect_delay(3000) // Set reconnect delay to 3 seconds
    .build()?;

let client = stock_options::StockOptionsWsStreams::production(configuration);
let connection = client.connect().await?;
let params = OrderReportV2StreamParams::builder("your-listen-key".to_string()).build()?;
let stream = connection.order_report_v2_stream(params).await?;
```
