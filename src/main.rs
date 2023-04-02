use waterlevel_server::runserver;

#[tokio::main]
async fn main() {
    let database_uri = dotenvy::var("DATABASE_URL").expect("Set database_url env variable");

    runserver(&database_uri).await;
}
