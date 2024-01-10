pub struct Config {
    pub addr: String,
    pub database_url: String
}

impl Config {
    pub fn env_config() -> Config {
       let addr = std::env::var("ADDR")
            .expect("KINDLY SET THE PORT");
        let database_url = std::env::var("DATABASE_URL")
            .expect("KINDLY SET THE DATABASE URL");
    
        Config {
            addr,
            database_url
        } 
    }
}