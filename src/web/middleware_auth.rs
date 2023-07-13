use crate::context::Context;
use crate::error::{Error, Result};
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn middleware_require_auth<T>(
    ctx: Result<Context>,
    req: Request<T>,
    next: Next<T>,
) -> Result<Response> {
    println!("->> {:<12} - middleware_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn middleware_context_resolve<T>(
    _state: State<ModelController>,
    cookies: Cookies,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response> {
    println!("->> {:<12} - middleware_context_resolve", "MIDDLEWARE");

    let ctx = match cookies
        .get(AUTH_TOKEN)
        .map(|token| token.value().to_string())
        .ok_or(Error::AuthFailedEmptyTokenCookie)
        .and_then(parse_auth_token)
    {
        Ok((user_id, _expiration, _signature)) => Ok(Context::new(user_id)),
        Err(e) => Err(e),
    };

    // Parse auth token from cookies successfully, but some of them are invalid
    // -> Auth Token is invalid, need to clean cookies to avoid wrong process in next middleware or route
    if ctx.is_err() && !matches!(ctx, Err(Error::AuthFailedInvalidUserIdTokenType)) {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    req.extensions_mut().insert(ctx);

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
        println!("->> {:<12} - Context", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Context>>()
            .ok_or(Error::AuthFailedEmptyCookie)?
            .clone()
    }
}

fn parse_auth_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, expiration, signature) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
            .ok_or(Error::AuthFailedInvalidUserIdTokenType)?;

    let user_id = user_id
        .parse()
        .map_err(|_| Error::AuthFailedInvalidUserIdTokenType)?;

    Ok((user_id, expiration.to_string(), signature.to_string()))
}
