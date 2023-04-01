use tokio::main;
use waterlevel_server::runserver;


#[tokio::main]
async fn main() {
    runserver().await;
}
