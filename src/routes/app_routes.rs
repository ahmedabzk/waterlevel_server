use axum::{
    routing::{get, post},
    Router,
};

use crate::app_state::AppState;

use tower_http::cors::{Any, CorsLayer};

use crate::{controllers::{registration::register, login::login},};

pub async fn create_routes(app_state: AppState) -> Router<()> {
    let cors = CorsLayer::new().allow_origin(Any);

    Router::new()
        .route("/hello", get(|| async { "Hello! World" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(cors)
        .with_state(app_state)
}
