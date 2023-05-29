mod controllers;
use axum::{
    routing::{get, post},
    Router,
};
use controllers::{entities::state::AppState, users::create_user};
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pg_host = match env::var_os("POSTGRES_HOST") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$POSTGRES_HOST is not set"),
    };
    let pg_port = match env::var_os("POSTGRES_PORT") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$POSTGRES_PORT is not set"),
    };
    let pg_user = match env::var_os("POSTGRES_USER") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$POSTGRES_USER is not set"),
    };
    let pg_password = match env::var_os("POSTGRES_PASSWORD") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$POSTGRES_PASSWORD is not set"),
    };
    let pg_url = format!(
        "postgres://{}:{}@{}:{}",
        pg_user, pg_password, pg_host, pg_port
    );

    let app_state = AppState::init_database(&pg_url).await.unwrap();
    let shared_state = Arc::new(RwLock::new(app_state));
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .with_state(shared_state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
