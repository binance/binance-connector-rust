# Error Handling

```rust
use binance_sdk::staking;
use binance_sdk::config;
use binance_sdk::errors;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .build()?;

let client = staking::StakingRestApi::production(configuration);
let params = staking::rest_api::ClaimBoostRewardsParams::default();

match client.claim_boost_rewards(params).await {
    Ok(response) => response,
    Err(e) => {
        if let Some(conn_err) = e.downcast_ref::<errors::ConnectorError>() {
            match conn_err {
                errors::ConnectorError::ConnectorClientError { msg, .. } => {
                    eprintln!("Client error: Check your request parameters. {}", msg);
                }
                errors::ConnectorError::UnauthorizedError { msg, .. } => {
                    eprintln!("Unauthorized: Invalid API credentials. {}", msg);
                }
                errors::ConnectorError::ForbiddenError { msg, .. } => {
                    eprintln!("Forbidden: Check your API key permissions. {}", msg);
                }
                errors::ConnectorError::TooManyRequestsError { msg, retry_after, .. } => {
                    eprintln!("Rate limit exceeded. Retry after {:?}s. {}", retry_after, msg);
                }
                errors::ConnectorError::RateLimitBanError { msg, retry_after, .. } => {
                    eprintln!("IP address banned due to excessive rate limits. Retry after {:?}s. {}", retry_after, msg);
                }
                errors::ConnectorError::ServerError { msg, status_code, .. } => {
                    eprintln!("Server error: {} (status code: {:?})", msg, status_code);
                }
                errors::ConnectorError::NetworkError(msg) => {
                    eprintln!("Network error: Check your internet connection. {}", msg);
                }
                errors::ConnectorError::NotFoundError { msg, .. } => {
                    eprintln!("Resource not found. {}", msg);
                }
                errors::ConnectorError::BadRequestError { msg, .. } => {
                    eprintln!("Bad request: Verify your input parameters. {}", msg);
                }
                errors::ConnectorError::ConflictError { msg, .. } => {
                    eprintln!("Conflict: {}", msg);
                }
                other => {
                    eprintln!("Unexpected ConnectorError variant: {:?}", other);
                }
            }

            if let Some(data) = conn_err.data() {
                eprintln!("Error data: {}", data);
            }
        } else {
            eprintln!("An unexpected error occurred: {:#}", e);
        }

        return Err(e);
    }
};
```
