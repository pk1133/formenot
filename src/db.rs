// This file sets up a database connection pool.
// Depending on whether you use Diesel or SQLx, the implementation will differ.
// Here’s a pseudo-code example using SQLx with PostgreSQL:

use sqlx::postgres::PgPoolOptions;

pub type DbPool = sqlx::Pool<sqlx::Postgres>;

pub fn init_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://myuser:mypass@localhost/mydb".to_string());
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create pool")
}
