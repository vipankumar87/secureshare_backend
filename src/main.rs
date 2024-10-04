use sqlx::postgres::PgPoolOptions;

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
