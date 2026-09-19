use axum::{
    routing::{get},
    Router
};
use location_service::features::create_location::create_location_command::CreateLocationCommand;

#[tokio::main]
async fn main () {
    let database_pool = location_service::infrastructure::db::create_db_pool().await.unwrap();
    let app = Router::new().route("/", get(CreateLocationCommand));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

