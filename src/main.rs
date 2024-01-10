use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{
    mysql::MySqlPoolOptions, Pool, MySql
};

mod user;
use user:: { model, schema, handlers, route };

pub struct AppState {
    db: Pool<MySql>
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("KINDLY SET THE DATABASE URL");

    let pool = match MySqlPoolOptions::new()
        .max_connections(69)
        .connect(&database_url)
        .await {
            Ok(pool) => {
                println!("Data Be Based");
                pool
            }
            Err(err) => {
                println!("Data Be Soy: {}", err);
                std::process::exit(1);
            }  
    };

    let app = route::create_router(Arc::new(AppState { db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}