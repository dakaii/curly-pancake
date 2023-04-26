use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

async fn init_database() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await?;

    Ok(pool)
}
