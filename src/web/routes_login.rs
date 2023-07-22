use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String
}

pub fn routes() -> Router {
  Router::new()
    .route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
  println!("Do API Login");

  if payload.username != "demo" || payload.pwd != "123" {
      return Err(Error::LoginFail);
  }

  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  Ok(body)
}