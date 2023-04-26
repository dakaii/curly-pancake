use sqlx::{Pool, Postgres};
pub struct State {
    pub connection: Pool<Postgres>,
}
