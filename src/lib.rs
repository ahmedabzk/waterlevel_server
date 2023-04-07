use axum::Extension;

mod controllers;
mod errors;
mod middlewares;
mod models;
mod routes;

use crate::routes::routes::create_routes;

use sqlx::postgres::PgPoolOptions;

pub async fn run(database_uri: &str) {
    let pool = PgPoolOptions::new()
        .connect(database_uri)
        .await
        .expect("unable to connect to database");

    let app = create_routes(Extension(pool)).await;

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    // tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
