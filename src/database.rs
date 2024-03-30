use sqlx::{
    mysql::MySqlPoolOptions,
    Pool,
    MySql
};

pub async fn db_connection(database_url: &str) -> Pool<MySql>{
    match MySqlPoolOptions::new()
        .max_connections(69)
        .connect(database_url)
        .await {
            Ok(pool) => {
                println!("Data Be Based");
                pool
            }
            Err(err) => {
                dbg!("Data Be Soy: {}", err);
                std::process::exit(1);
            }  
    }
}
