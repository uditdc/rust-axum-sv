use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use surrealdb::sql::Thing;

use crate::{Result, DB};

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String
}

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
    title: String,
    marketing: bool
}

pub fn routes() -> Router {
  Router::new()
    .route("/api/login", post(api_login))
}

async fn api_login(_payload: Json<LoginPayload>) -> Result<Json<Value>> {
  println!("Do API Login");

  let people: Vec<Record> = DB.select("person").await.expect("Cant get list");

  let body = Json(json!({
    "result": {
      "success": true,
      "people": people
    }
  }));

  Ok(body)
}