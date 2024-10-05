#[derive(Debug, Clone)]

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config{
        let database_url: String =  std::env::var("DATABASE_URL").expect("Database url must have valid connection string");
        let jwt_secret: String = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must have valid");
        let jwt_maxage: String = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must have in .env");

        Config {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port: 8000
        }
    }
}