use crate::context::Context;
use crate::error::{Error, Result};
use crate::web::AUTH_TOKEN;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn middleware_require_auth<T>(
    ctx: Result<Context>,
    req: Request<T>,
    next: Next<T>,
) -> Result<Response> {
    println!("->> {:<12} - middleware_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

#[async_trait]
// S need to implement Send and Sync to ensure thread safety
// FromRequestParts extracts request data
// - allow to extract parts of request data and transform it to another type
// - in this case we extract Cookies part of request, and transform it to Context
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies
            .get(AUTH_TOKEN)
            .map(|token| token.value().to_string());

        let (user_id, _expiration, _signature) = auth_token
            .ok_or(Error::AuthFailedEmptyTokenCookie)
            .and_then(parse_token)?;

        Ok(Context::new(user_id))
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, expiration, signature) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
            .ok_or(Error::AuthFailedInvalidToken)?;

    let user_id = user_id.parse().map_err(|_| Error::AuthFailedInvalidToken)?;

    Ok((user_id, expiration.to_string(), signature.to_string()))
}
