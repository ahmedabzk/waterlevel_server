use axum::{
    Router,
    routing::{get},
};

pub async fn create_routes() -> Router<()>{

    Router::new()
        .route("/hello", get(|| async {
            "Hello! World"
        }))
}