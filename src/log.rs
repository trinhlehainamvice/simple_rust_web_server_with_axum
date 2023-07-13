use crate::context::Context;
use crate::error::{ClientError, Error, Result};
use axum::http::{Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use std::time::SystemTime;

pub async fn request(
    uuid: String,
    req_method: Method,
    uri: Uri,
    ctx: Option<Context>,
    service_error: Option<&Error>,
    client_status_error: Option<(StatusCode, ClientError)>,
) -> Result<Response> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let client_res = client_status_error.as_ref().map(|(status, error)| {
        let error_body = json!({
            "error": {
                "type": error.as_ref(),
                "request_uuid": uuid,
            }
        });

        (*status, Json(error_body)).into_response()
    });

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let client_error = client_status_error.unzip().1;

    let log = RequestLog {
        uuid,
        timestamp,
        user_id: ctx.map(|ctx| ctx.user_id),
        req_uri: uri.to_string(),
        req_method: req_method.to_string(),
        client_error_type: client_error.map(|ce| ce.as_ref().to_string()),
        error_type,
        error_data,
    };

    // TODO: Post Log to metric (log) server
    println!("      ->> log:\n{}", json!(log));
    println!();

    client_res.ok_or(Error::FailedToRequestLog)
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLog {
    uuid: String,
    timestamp: String,

    // User
    user_id: Option<u64>,

    // http request
    req_uri: String,
    req_method: String,

    // Error
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
