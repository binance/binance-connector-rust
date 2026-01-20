# Error Handling

```rust
use binance_sdk::derivatives_trading_options;
use binance_sdk::config;
use binance_sdk::errors;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .build()?;

let client = derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesRestApi::production(configuration);

match client.exchange_information().await {
    Ok(response) => response,
    Err(e) => {
        if let Some(conn_err) = e.downcast_ref::<errors::ConnectorError>() {
            match conn_err {
                errors::ConnectorError::ConnectorClientError { msg, code } => {
                    eprintln!("Client error: Check your request parameters. {}", msg);
                }
                errors::ConnectorError::UnauthorizedError { msg, code } => {
                    eprintln!("Unauthorized: Invalid API credentials. {}", msg);
                }
                errors::ConnectorError::ForbiddenError { msg, code } => {
                    eprintln!("Forbidden: Check your API key permissions. {}", msg);
                }
                errors::ConnectorError::TooManyRequestsError { msg, code } => {
                    eprintln!("Rate limit exceeded. Please wait and try again. {}", msg);
                }
                errors::ConnectorError::RateLimitBanError { msg, code } => {
                    eprintln!("IP address banned due to excessive rate limits. {}", msg);
                }
                errors::ConnectorError::ServerError { msg, status_code } => {
                    eprintln!("Server error: {} (status code: {:?})", msg, status_code);
                }
                errors::ConnectorError::NetworkError { msg, code } => {
                    eprintln!("Network error: Check your internet connection. {}", msg);
                }
                errors::ConnectorError::NotFoundError { msg, code } => {
                    eprintln!("Resource not found. {}", msg);
                }
                errors::ConnectorError::BadRequestError { msg, code } => {
                    eprintln!("Bad request: Verify your input parameters. {}", msg);
                }
                other => {
                    eprintln!("Unexpected ConnectorError variant: {:?}", other);
                }
            }
        } else {
            eprintln!("An unexpected error occurred: {:#}", e);
        }

        return Err(e);
    }
};
```
