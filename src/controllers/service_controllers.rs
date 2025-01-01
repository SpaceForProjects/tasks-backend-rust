use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use hyper::StatusCode;
use serde_json::json;

pub async fn status() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");

    let response = json!({
        "data": {
            "version": version,
        },
        "message": "Service is running ..."
    });

    (StatusCode::OK, Json(response))
}

pub fn router() -> Router {
    Router::new().route("/api/status", get(status))
}
