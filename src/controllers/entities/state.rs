use axum::extract::FromRef;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(app_state: &AppState) -> Pool<Postgres> {
        app_state.pool.clone()
    }
}

impl AppState {
    pub async fn init_database(pg_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(pg_url)
            .await
            .expect("couldn't connect to the database");

        Ok(AppState { pool })
    }
}
