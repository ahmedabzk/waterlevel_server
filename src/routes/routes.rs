use axum::{routing::{get, post}, Extension, Router};

use sqlx::postgres::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tracing::callsite::register;

mod user_controller;

use user_controller::register;

pub async fn create_routes(Extension(database): Extension<PgPool>) -> Router<()> {
    let cors = CorsLayer::new().allow_origin(Any);

    Router::new()
        .route("/hello", get(|| async { "Hello! World" }))
        .route("/register", post(register))
        .layer(cors)
}
