use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{ Pool, MySql };

mod database;
mod config;
mod router;

mod user;
use user::{ schema, handlers };

pub struct AppState {
    db: Pool<MySql>
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = config::Config::env_config();
    
    let pool = database::db_connection(&config.database_url).await;

    let app = router::create_router(Arc::new(AppState { db: pool.clone() })).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}