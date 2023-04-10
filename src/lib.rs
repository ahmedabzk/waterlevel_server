use app_state::AppState;


mod controllers;
mod errors;
mod middlewares;
mod models;
mod routes;
pub mod utilis;
pub mod app_state;

use crate::routes::app_routes::create_routes;


pub async fn run(app_state: AppState) {

  

    let app = create_routes(app_state).await;

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    // tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
