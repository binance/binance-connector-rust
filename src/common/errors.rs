use thiserror::Error;
use tokio_tungstenite::tungstenite::error::ProtocolError;

/// Represents different types of WebSocket connection failures and their reconnection eligibility
#[derive(Debug, Clone, Copy)]
pub enum WebsocketConnectionFailureReason {
    /// Network-level interruption (should reconnect)
    NetworkInterruption,
    /// Connection reset by peer (should reconnect)
    ConnectionReset,
    /// Server temporary error (should reconnect)
    ServerTemporaryError,
    /// Unexpected connection close (should reconnect)
    UnexpectedClose,
    /// Stream ended unexpectedly (should reconnect)
    StreamEnded,
    /// Authentication/authorization failure (should not reconnect)
    AuthenticationFailure,
    /// Protocol violation (should not reconnect)
    ProtocolViolation,
    /// Configuration error (should not reconnect)
    ConfigurationError,
    /// User initiated close (should not reconnect)
    UserInitiatedClose,
    /// Permanent server error (should not reconnect)
    PermanentServerError,
    /// Normal close (should not reconnect)
    NormalClose,
}

impl WebsocketConnectionFailureReason {
    /// Classifies a tungstenite error into a failure reason
    pub fn from_tungstenite_error(error: &tokio_tungstenite::tungstenite::Error) -> Self {
        use tokio_tungstenite::tungstenite::Error;
        match error {
            Error::ConnectionClosed | Error::AlreadyClosed => Self::ConnectionReset,
            Error::Io(io_error) => {
                use std::io::ErrorKind;
                match io_error.kind() {
                    ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted => {
                        Self::ConnectionReset
                    }
                    ErrorKind::UnexpectedEof => Self::StreamEnded,
                    ErrorKind::PermissionDenied => Self::AuthenticationFailure,
                    _ => Self::NetworkInterruption,
                }
            }
            Error::Tls(_) | Error::Capacity(_) | Error::Url(_) => Self::ConfigurationError,
            Error::Protocol(ProtocolError::ResetWithoutClosingHandshake) => Self::UnexpectedClose,
            Error::Protocol(_) | Error::Utf8 | Error::HttpFormat(_) => Self::ProtocolViolation,
            Error::Http(_) => Self::ServerTemporaryError,
            _ => Self::NetworkInterruption,
        }
    }

    /// Classifies a WebSocket close code into a failure reason
    #[must_use]
    pub fn from_close_code(code: u16, user_initiated: bool) -> Self {
        if user_initiated {
            return Self::UserInitiatedClose;
        }
        match code {
            1000 => Self::NormalClose,                               // Normal closure
            1001 | 1011 | 1012 | 1013 => Self::ServerTemporaryError, // Server temporary issues
            1002 | 1003 | 1007 | 1009 | 1010 => Self::ProtocolViolation, // Protocol violations
            1008 | 1014 | 4000..=4999 => Self::PermanentServerError, // Permanent server errors
            1015 => Self::ConfigurationError,                        // TLS handshake failure
            _ => Self::UnexpectedClose,                              // Unknown codes including 1006
        }
    }

    /// Determines if this failure type warrants a reconnection attempt
    #[must_use]
    pub fn should_reconnect(&self) -> bool {
        match self {
            // Reconnectable failures
            Self::NetworkInterruption
            | Self::ConnectionReset
            | Self::ServerTemporaryError
            | Self::UnexpectedClose
            | Self::StreamEnded => true,
            // Non-reconnectable failures
            Self::AuthenticationFailure
            | Self::ProtocolViolation
            | Self::ConfigurationError
            | Self::UserInitiatedClose
            | Self::PermanentServerError
            | Self::NormalClose => false,
        }
    }
}

/// Errors returned by the REST API client.
///
/// Variants that describe an error response from Binance carry `body`: the
/// full parsed JSON body, when the body is valid JSON. Use [`ConnectorError::data`]
/// to read it, or [`ConnectorError::body`] for the whole body.
///
/// The enum and its response variants are `#[non_exhaustive]`, so match with a
/// wildcard arm and destructure variants with `..`.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ConnectorError {
    /// Errors raised by the client itself (request building, decoding, ...)
    /// and HTTP error statuses without a dedicated variant. `body` is set only
    /// for the latter.
    #[error("Connector client error: {msg}")]
    #[non_exhaustive]
    ConnectorClientError {
        msg: String,
        code: Option<i64>,
        body: Option<serde_json::Value>,
    },

    #[error("Unauthorized access. Authentication required. {msg}")]
    #[non_exhaustive]
    UnauthorizedError {
        msg: String,
        code: Option<i64>,
        body: Option<serde_json::Value>,
    },

    #[error("Access to the requested resource is forbidden. {msg}")]
    #[non_exhaustive]
    ForbiddenError {
        msg: String,
        code: Option<i64>,
        body: Option<serde_json::Value>,
    },

    #[error("Too many requests. You are being rate-limited. {msg}")]
    #[non_exhaustive]
    TooManyRequestsError {
        msg: String,
        code: Option<i64>,
        retry_after: Option<u32>,
        body: Option<serde_json::Value>,
    },

    #[error("The IP address has been banned for exceeding rate limits. {msg}")]
    #[non_exhaustive]
    RateLimitBanError {
        msg: String,
        code: Option<i64>,
        retry_after: Option<u32>,
        body: Option<serde_json::Value>,
    },

    #[error("Internal server error: {msg} (status code: {status_code:?})")]
    #[non_exhaustive]
    ServerError {
        msg: String,
        status_code: Option<u16>,
        body: Option<serde_json::Value>,
    },

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("The requested resource was not found. {msg}")]
    #[non_exhaustive]
    NotFoundError {
        msg: String,
        code: Option<i64>,
        body: Option<serde_json::Value>,
    },

    #[error("Bad request: {msg}")]
    #[non_exhaustive]
    BadRequestError {
        msg: String,
        code: Option<i64>,
        body: Option<serde_json::Value>,
    },

    #[error("Conflict: {msg}")]
    #[non_exhaustive]
    ConflictError {
        msg: String,
        code: Option<i64>,
        body: Option<serde_json::Value>,
    },
}

impl ConnectorError {
    #[must_use]
    pub fn retry_after(&self) -> Option<u32> {
        match self {
            Self::TooManyRequestsError { retry_after, .. }
            | Self::RateLimitBanError { retry_after, .. } => *retry_after,
            _ => None,
        }
    }

    /// The full parsed JSON body of the error response (including `code` and
    /// `msg`), if Binance returned a JSON body.
    #[must_use]
    pub fn body(&self) -> Option<&serde_json::Value> {
        match self {
            Self::ConnectorClientError { body, .. }
            | Self::UnauthorizedError { body, .. }
            | Self::ForbiddenError { body, .. }
            | Self::TooManyRequestsError { body, .. }
            | Self::RateLimitBanError { body, .. }
            | Self::ServerError { body, .. }
            | Self::NotFoundError { body, .. }
            | Self::BadRequestError { body, .. }
            | Self::ConflictError { body, .. } => body.as_ref(),
            Self::NetworkError(_) => None,
        }
    }

    /// The `data` object of the error response body, if present.
    ///
    /// For `order.cancelReplace` this holds `cancelResult`, `newOrderResult`,
    /// `cancelResponse` and `newOrderResponse`.
    #[must_use]
    pub fn data(&self) -> Option<&serde_json::Value> {
        self.body()
            .and_then(|b| b.get("data"))
            .filter(|d| !d.is_null())
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WebsocketError {
    #[error("WebSocket timeout error")]
    Timeout,
    #[error("WebSocket protocol error: {0}")]
    Protocol(String),
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("WebSocket handshake error: {0}")]
    Handshake(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("No active WebSocket connection error.")]
    NotConnected,
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("No response error.")]
    NoResponse,
    #[error("Server‐side response error (code {code}): {message}")]
    #[non_exhaustive]
    ResponseError {
        code: i64,
        message: String,
        retry_after: Option<i64>,
        data: Option<serde_json::Value>,
    },
}
