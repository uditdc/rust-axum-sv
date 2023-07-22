use axum::{
    routing::{get, get_service},
    response::Json,
    Router,
    extract::{Query, Path}
};
use serde::Deserialize;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Deserialize)]
struct RouteParams {
    name: Option<String>
}

async fn query_param(Query(params): Query<RouteParams>) -> Json<serde_json::Value> {
    let name = params.name.as_deref().unwrap_or("HLO");
    Json(serde_json::json!({ "query": name }))
}

async fn path_param(Path(name): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "path": name }))
}

fn routes_test() -> Router {
    return Router::new()
        .route("/", get(hello_world))
        .route("/query-param", get(query_param))
        .route("/path-param/:name", get(path_param))
}

fn routes_static() -> Router {
    return Router::new()
        .nest_service("/", get_service(ServeDir::new("./")))
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(routes_test())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    Ok(router.into())
}
