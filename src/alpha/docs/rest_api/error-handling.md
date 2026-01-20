# Error Handling

```rust
use binance_sdk::alpha;
use binance_sdk::config;
use binance_sdk::errors;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .build()?;

let client = alpha::AlphaRestApi::production(configuration);

match client.get_exchange_info(params).await {
    Ok(response) => response,
    Err(e) => {
        if let Some(conn_err) = e.downcast_ref::<errors::ConnectorError>() {
            match conn_err {
                errors::ConnectorError::ConnectorClientError(msg) => {
                    eprintln!("Client error: Check your request parameters. {}", msg);
                }
                errors::ConnectorError::UnauthorizedError(msg) => {
                    eprintln!("Unauthorized: Invalid API credentials. {}", msg);
                }
                errors::ConnectorError::ForbiddenError(msg) => {
                    eprintln!("Forbidden: Check your API key permissions. {}", msg);
                }
                errors::ConnectorError::TooManyRequestsError(msg) => {
                    eprintln!("Rate limit exceeded. Please wait and try again. {}", msg);
                }
                errors::ConnectorError::RateLimitBanError(msg) => {
                    eprintln!("IP address banned due to excessive rate limits. {}", msg);
                }
                errors::ConnectorError::ServerError { msg, status_code } => {
                    eprintln!("Server error: {} (status code: {:?})", msg, status_code);
                }
                errors::ConnectorError::NetworkError(msg) => {
                    eprintln!("Network error: Check your internet connection. {}", msg);
                }
                errors::ConnectorError::NotFoundError(msg) => {
                    eprintln!("Resource not found. {}", msg);
                }
                errors::ConnectorError::BadRequestError(msg) => {
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
