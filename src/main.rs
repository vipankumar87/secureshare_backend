use sqlx::postgres::PgPoolOptions;
mod config;
mod model;
mod dtos;
mod error;
mod db;
// #[derive(Debug,Clone)]
// struct Person {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let person1 = Person { name: "Alice".to_string(), age: 30 };
//     let person2 = person1.clone();  // Creates a deep copy of `person1`
//     println!("{:?}", person2);  // Outputs the cloned `person2`
// }

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "postgresql://rudra:Dragon123@223.177.161.92:5432/sharer";
    
    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("Connected to PostgreSQL!");

    Ok(())
}
