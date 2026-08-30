// Basic health and root endpoints

use axum::{
    http::{StatusCode, header}, response::IntoResponse, 
};
use serde_json::{json};

pub async fn root() -> impl IntoResponse {
    let res = json!({
        "application": "acmcsufossbot",
        "status": "running",
        "endpoints": {
            "health": "/health"
        }
    });

    let mut res_string = match serde_json::to_string(&res) {
        Ok(s) => s,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Serilization failed").into_response()
    };

    // new line or terminal clobbering
    // (should probably ask if this is normal?)
    res_string.push('\n');

    (
    StatusCode::OK,
    [(header::CONTENT_TYPE, "application/json")],
    res_string,
    ).into_response()
}

pub async fn health() ->  impl IntoResponse { 
    let res = json!({
        "status": "ok",
        "message": "healthy"
    });

    let mut res_string = match serde_json::to_string(&res) {
        Ok(s) => s,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Serilization failed").into_response()
    };

    res_string.push('\n');

    (
    StatusCode::OK,
    [(header::CONTENT_TYPE, "application/json")],
    res_string,
    ).into_response()
}



