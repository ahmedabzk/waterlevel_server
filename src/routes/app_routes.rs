use axum::{
    routing::{get, post},
    Router,
    middleware,
};

use crate::{app_state::AppState};
use crate::middlewares::middleware::require_auth;

use tower_http::cors::{Any, CorsLayer};

use crate::controllers::auth::{registration::register, login::login};
use crate::controllers::stats::{create_stats::post_stats};


pub async fn create_routes(app_state: AppState) -> Router<()> {
    let cors = CorsLayer::new().allow_origin(Any);

    Router::new()
        .route("/hello", get(|| async { "Hello! World" }))
        .route("/api/v1/createStats", post(post_stats))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(), 
            require_auth
        ))
        .route("/api/v1/register", post(register))
        .route("/api/v1/login", post(login))
        .layer(cors)
        .with_state(app_state)
}
