use axum::{
    routing::get,
    response::Json,
    Router
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn success() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "success": true }))
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/success", get(success));

    Ok(router.into())
}
