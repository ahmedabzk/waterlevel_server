use sqlx::postgres::PgPoolOptions;
use waterlevel_server::{run, app_state::AppState, utilis::token_wrapper::TokenWrapper};


#[tokio::main]
async fn main() {
    let database_uri = dotenvy::var("DATABASE_URL")
        .expect("Set database_url env variable");
    let jwt_secret = dotenvy::var("SECRET")
        .expect("failed to set jwt_secret");

    let db = PgPoolOptions::new()
        .connect(&database_uri)
        .await
        .expect("unable to connect to database");

    let app_state = AppState{
        db,
        jwt_secret: TokenWrapper(jwt_secret),

    };

    run(app_state).await;
}
