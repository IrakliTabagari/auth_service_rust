mod permissions;

use axum::{
    Json,
    extract::{Extension, Path},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::Deserialize;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use permissions::service::seed_permissions;

#[tokio::main]
async fn main() {

    let db = Surreal::new::<Mem>(()).await.unwrap();

    db.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();

    db.use_ns("namespace").use_db("database").await.unwrap();

    let db_arc = Arc::new(db);
    seed_permissions(db_arc.clone()).await.expect("Failed to seed permissions");

    // build our application with a single route
    let app = Router::new()
        .nest("/permissions", permissions::api::api_routes())
        // .nest("/permissions", permissions::service::get_root())
        .layer(Extension(Arc::new(db)));
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct AppState {
    db: Surreal<Mem>,
}