// TODO: This file will be renamed soon to fit what purpose it serves
// There should not be a gigantic monofile with all our github logic 
use crate::http::AppState;
use axum::{extract::State, http::{StatusCode, header}, response::IntoResponse};

// this is like supposed to be an example, only works with prs ofc
// if you try getting an issue then nothing is returned
pub async fn prget(State(state): State<AppState>) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let pr = state.octo
    .pulls("acmcsufoss", "acmcsuf.com")
    .get(2)
    .await
    .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to get PR"
        ))?;
    
    let mut pr_str = match serde_json::to_string(&pr) {
        Ok(s) => s,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Unable to serialize pr"))
    };

    pr_str.push('\n');

    Ok((
    StatusCode::OK,
    [(header::CONTENT_TYPE, "application/json")],
    pr_str
    ).into_response())
}
