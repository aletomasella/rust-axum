use crate::{Error, Result};
use axum::{response::IntoResponse, routing::post, Json, Router};
use serde_json::json;

#[derive(Debug, serde::Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<impl IntoResponse> {
    println!("{:?}", payload);

    if (payload.username != "admin" || payload.password != "admin") {
        return Err(Error::LoginFail);
    }

    let response = Json(json!({
        "status": "success",
        "message": "Login Success",
    }));

    Ok(response)
}
