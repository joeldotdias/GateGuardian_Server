pub struct Config {
    pub socket_addr: String,
    pub database_url: String,
}

impl Config {
    pub fn env_config() -> Self {
       let socket_addr = std::env::var("SOCKET_ADDR")
            .expect("KINDLY SET THE PORT");
        let database_url = std::env::var("DATABASE_URL")
            .expect("KINDLY SET THE DATABASE URL");
          
        return Config {
            socket_addr,
            database_url
        };
    }
}
