use thiserror::Error;

/// Represents different types of WebSocket connection failures and their reconnection eligibility
#[derive(Debug, Clone)]
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

#[derive(Error, Debug)]
pub enum ConnectorError {
    #[error("Connector client error: {0}")]
    ConnectorClientError(String),

    #[error("Unauthorized access. Authentication required. {0}")]
    UnauthorizedError(String),

    #[error("Access to the requested resource is forbidden. {0}")]
    ForbiddenError(String),

    #[error("Too many requests. You are being rate-limited. {0}")]
    TooManyRequestsError(String),

    #[error("The IP address has been banned for exceeding rate limits. {0}")]
    RateLimitBanError(String),

    #[error("Internal server error: {msg} (status code: {status_code:?})")]
    ServerError {
        msg: String,
        status_code: Option<u16>,
    },

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("The requested resource was not found. {0}")]
    NotFoundError(String),

    #[error("Bad request: {0}")]
    BadRequestError(String),
}

#[derive(Debug, Error)]
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
    ResponseError { code: i64, message: String },
}
