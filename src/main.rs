use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router, Server};
use learn_rust_web_server::context::Context;
use learn_rust_web_server::error::Error;
use learn_rust_web_server::model::ModelController;
use learn_rust_web_server::web::middleware_auth::{
    middleware_context_resolve, middleware_require_auth,
};
use learn_rust_web_server::{log, web};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mc = ModelController::new();

    // Use route_layer to wrap middleware around only this route
    let ticket_router = web::ticket_router::route(mc.clone())
        .route_layer(middleware::from_fn(middleware_require_auth));

    let router = Router::new()
        .merge(hello_routers())
        .merge(web::login_router::route())
        .nest("/api", ticket_router)
        // RESPONSE Layer Tail
        .layer(middleware::map_response(main_response_mapper))
        // RESPONSE Layer Head
        // REQUEST Layer Tail
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            middleware_context_resolve,
        ))
        .layer(CookieManagerLayer::new())
        // REQUEST Layer Head
        .fallback_service(static_routers());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("LISTENING on {addr}");
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap()
}

async fn main_response_mapper(
    ctx: Option<Context>,
    method: Method,
    uri: Uri,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "MAP_RESPONSE");
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    log::request(
        Uuid::new_v4().to_string(),
        method,
        uri,
        ctx,
        service_error,
        client_status_error,
    )
    .await
    .unwrap_or(res)
}

fn static_routers() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn hello_routers() -> Router {
    Router::new()
        .route("/hello", get(hello_query_handler))
        .route("/hello/:name", get(hello_extract_path_handler))
}

// Query <=> ?
// /hello?name=[params]
async fn hello_query_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - hello_query_handler - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello {name}</h1>"))
}

// Path <=> /
// /hello/:name
// Example: /hello/Alex -> Path<String> <=> extract Alex as String and pass to name
async fn hello_extract_path_handler(Path(name): Path<String>) -> impl IntoResponse {
    println!(
        "->> {:<12} - hello_extract_path_handler - {name:?}",
        "HANDLER"
    );
    Html(format!("<h1>Hello {name}</h1>"))
}
