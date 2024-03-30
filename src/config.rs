use sqlx::{ Pool, MySql };
use dotenv::dotenv;

pub struct Config {
    pub socket_addr: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let socket_addr = std::env::var("SOCKET_ADDR")
            .expect("KINDLY SET THE PORT");
        let database_url = std::env::var("DATABASE_URL")
            .expect("KINDLY SET THE DATABASE URL");
          
        Config {
            socket_addr,
            database_url
        }
    }
}


pub struct AppState {
    pub db: Pool<MySql>
}

impl AppState {
    pub fn new(db: Pool<MySql>) -> Self {
        Self { db }
    }
}
