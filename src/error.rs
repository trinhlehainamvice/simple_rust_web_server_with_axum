use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LoginFailed,
    AuthFailedEmptyTokenCookie,
    AuthFailedEmptyCookie,
    AuthFailedInvalidUserIdTokenType,
    DeleteTicketFailed { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:<12} - {self:?}", "Error::into_response");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
