use std::sync::Arc;

use crate::controllers::entities::user::{CreateUser, User};
use axum::{extract::State, http::StatusCode, Json};

use super::entities::state::AppState;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>)> {
    // insert your application logic here
    let countries = sqlx::query!(
        "
SELECT country, COUNT(*) as count
FROM users
GROUP BY country
WHERE organization = ?
        ",
        organization
    )
    .fetch_all(&pool) // -> Vec<{ country: String, count: i64 }>
    .await?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(StatusCode::CREATED, Json(user))
}
