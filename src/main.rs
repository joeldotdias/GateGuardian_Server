use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{ Pool, MySql };

mod config;
mod database;
mod middleware;
mod router;

mod user;
mod resident;

pub struct AppState {
    db: Pool<MySql>
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = config::Config::env_config();
    
    let pool = database::db_connection(&config.database_url).await;

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = router::create_router(Arc::new(AppState { db: pool.clone() })).await;

    let listener = tokio::net::TcpListener::bind(&config.socket_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
