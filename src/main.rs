use axum::{
    extract::{Path, Query},
    response::Json,
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use shuttle_secrets::SecretStore;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

pub static DB: Surreal<Client> = Surreal::init();

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Deserialize)]
struct RouteParams {
    name: Option<String>,
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
        .route("/path-param/:name", get(path_param));
}

fn routes_static() -> Router {
    return Router::new().nest_service("/", get_service(ServeDir::new("./")));
}

async fn connect_db(
    db_uri: String,
    db_username: String,
    db_password: String,
    db_namespace: String,
) -> surrealdb::Result<()> {
    // Connect to the server
    DB.connect::<Ws>(db_uri).await.expect("Failed to connect");

    DB.signin(Root {
        username: &db_username,
        password: &db_password,
    })
    .await.expect("Failed to signin");

    DB.use_ns(&db_namespace).use_db(&db_namespace).await.expect("Failed to set namespace");

    Ok(())
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Connect DB
    if let (Some(db_uri), Some(db_username), Some(db_password), Some(db_namespace)) = (
        secret_store.get("DB_URI"),
        secret_store.get("DB_USERNAME"),
        secret_store.get("DB_PASSWORD"),
        secret_store.get("DB_NAMESPACE"),
    ) {
        connect_db(db_uri, db_username, db_password, db_namespace)
            .await
            .expect("Had some errors running migrations :(");
    } else {
        eprintln!("Unable to retrieve all necessary secrets for database connection");
    }

    let router = Router::new()
        .merge(routes_test())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    Ok(router.into())
}
