
use axum::{
    Router,
    routing::{get, post},
    Json, Extension,
};
use crate::permissions::service::{get_permissions};
use crate::permissions::models::Permission;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use std::sync::Arc;

use crate::permissions::service as permissions_service;

pub fn api_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/list", get(list_permissions))
}

async fn root() {
    permissions_service::get_root().await;
}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}

async fn list_permissions(
    Extension(db): Extension<Arc<Surreal<Mem>>>,
) -> Result<Json<Vec<Permission>>, String> {
    match get_permissions(db).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(format!("Error: {}", err)),
    }
}