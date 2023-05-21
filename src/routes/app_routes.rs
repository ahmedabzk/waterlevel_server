use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::middlewares::middleware::require_auth;
use crate::{app_state::AppState, controllers::stats::get_stats::get_all_stats};

use tower_http::cors::{Any, CorsLayer};

use crate::controllers::auth::{login::login, registration::register};
use crate::controllers::stats::{create_stats::post_stats, get_one_stat::get_stat_by_id};

pub async fn create_routes(app_state: AppState) -> Router<()> {
    let cors = CorsLayer::new().allow_origin(Any);

    Router::new()
        .route("/hello", get(|| async { "Hello! World" }))
        .route("/api/v1/createStats", post(post_stats))
        .route("/api/v1/getStats", get(get_all_stats))
        .route("/api/v1/getStatById/:id", get(get_stat_by_id))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_auth,
        ))
        .route("/api/v1/register", post(register))
        .route("/api/v1/login", post(login))
        .layer(cors)
        .with_state(app_state)
}
