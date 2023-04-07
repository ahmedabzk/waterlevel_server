use waterlevel_server::run;

#[tokio::main]
async fn main() {
    let database_uri = dotenvy::var("DATABASE_URL").expect("Set database_url env variable");

    run(&database_uri).await;
}
