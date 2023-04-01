use axum::{
    Router,
    routing::{get},
};

use tower_http::cors::{CorsLayer, Any};
use sqlx::postgres::PgPoolOptions;

pub async fn create_routes(database_uri: ) -> Router<()>{

     let cors = CorsLayer::new().allow_origin(Any);

     let pool = PgPoolOptions::new()
        .connect(&d)

    Router::new()
        .route("/hello", get(|| async {
            "Hello! World"
        }))
        .layer(cors)

}