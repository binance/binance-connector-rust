# Error Handling

```rust
use binance_sdk::gift_card;
use binance_sdk::config;
use binance_sdk::errors;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .build()?;

let client = gift_card::GiftCardRestApi::production(configuration);
let params = gift_card::rest_api::CreateASingleTokenGiftCardParams::builder("6H9EKF5ECCWFBHGE".to_string(), 1000.0).build()?;

match client.create_a_single_token_gift_card(params).await {
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
