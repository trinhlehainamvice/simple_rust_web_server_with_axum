use crate::error::{Error, Result};
use crate::web::AUTH_TOKEN;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

pub async fn middleware_require_auth<T>(
    cookies: Cookies,
    req: Request<T>,
    next: Next<T>,
) -> Result<Response> {
    println!("->> {:<12} - middleware_require_auth", "MIDDLEWARE");

    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|token| token.value().to_string());

    auth_token.ok_or(Error::AuthFailedInvalidToken)?;

    Ok(next.run(req).await)
}
