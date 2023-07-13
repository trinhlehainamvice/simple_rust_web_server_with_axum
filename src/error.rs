use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

// NOTE: strum macros convert enum to &'static str
#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailed,
    AuthFailedEmptyTokenCookie,
    AuthFailedEmptyCookie,
    AuthFailedInvalidUserIdTokenType,
    DeleteTicketFailed { id: u64 },
    FailedToRequestLog,
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        // NOTE: we currently handle all pattern, so fallback will not be used or unreachable
        // So we mark unreachable_patterns macro here to avoid warning
        #[allow(unreachable_patterns)]
        match self {
            // Login
            Error::LoginFailed => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Auth
            Error::AuthFailedInvalidUserIdTokenType
            | Error::AuthFailedEmptyTokenCookie
            | Error::AuthFailedEmptyCookie => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            // Ticket Model
            Error::DeleteTicketFailed { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::SERVICE_ERROR)
            }

            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:<12} - {self:?}", "Error::into_response");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}

// NOTE: strum macros convert enum to &'static str
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
