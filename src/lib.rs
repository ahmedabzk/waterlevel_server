use axum::Extension;

mod routes;
mod middlewares;
mod controllers;
mod models;


use routes::routes::create_routes;

use sqlx::postgres::PgPoolOptions;


pub async fn runserver(database_uri: &str) {
    let pool = PgPoolOptions::new()
        .connect(&database_uri)
        .await
        .expect("unable to connect to database");

    let app = create_routes(Extension(pool));
    
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.await.into_make_service())
        .await
        .expect("failed to start server");
}
