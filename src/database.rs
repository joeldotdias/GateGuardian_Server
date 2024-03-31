use sqlx::{
    mysql::MySqlPoolOptions,
    Pool,
    MySql
};

pub async fn db_connection(database_url: &str) -> Pool<MySql> {
    match MySqlPoolOptions::new()
        .max_connections(69)
        .connect(database_url)
        .await {
            Ok(pool) => {
                println!("Data Be Based");
                match sqlx::migrate!("src/migrations").run(&pool).await {
                    Ok(_) => {
                        println!("Migrations performed successfully");
                    },
                    Err(err) => {
                        println!("Couldn't perform migrations: {}", err);
                    },
                };
                pool
            }
            Err(err) => {
                dbg!("Data Be Soy: {}", err);
                std::process::exit(1);
            }  
    }
}
