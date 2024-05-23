
use axum::{
    Router,
    routing::{get, post}
};
use crate::permissions::service as permissions_service;

pub fn api_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
}

async fn root() {
    permissions_service::get_root().await;
}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}
