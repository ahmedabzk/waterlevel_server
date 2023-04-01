use axum;

mod routes;

use routes::routes::create_routes;


pub async fn runserver(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();

    let app = create_routes(database);

    let addr = std::net::SocketAddr::from(([127,0,0,1], 3000));

    axum::Server::bind(&addr)
        .serve(app.await.into_make_service())
        .await
        .expect("failed to start server");
}