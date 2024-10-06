use dotenv::dotenv;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, PgConnection, Pool, Postgres};
mod config;
mod model;
mod dtos;
mod error;
mod db;

use config::Config;
use db::DBClient;
use tracing_subscriber::filter::LevelFilter;


#[derive(Debug,Clone)]
pub struct  AppState{
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();
    let config: Config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

}
// // #[derive(Debug,Clone)]
// // struct Person {
// //     name: String,
// //     age: u32,
// // }

// // fn main() {
// //     let person1 = Person { name: "Alice".to_string(), age: 30 };
// //     let person2 = person1.clone();  // Creates a deep copy of `person1`
// //     println!("{:?}", person2);  // Outputs the cloned `person2`
// // }

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     let database_url = "postgresql://rudra:Dragon123@223.177.161.92:5432/sharer";
    
//     let _pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await?;

//     println!("Connected to PostgreSQL!");

//     Ok(())
// }
