use axum::{
    routing::{get, post},
    Router,
};
use rusty_auth::controllers::users::create_user;
use std::net::SocketAddr;

#[derive(FromRef, Clone)]
pub struct AppState {
    pool: PgPool,
    client: HTTPClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = AppState { pool, client };
    let shared_state = Arc::new(RwLock::new(app_state));
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
