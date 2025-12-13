use axum::{routing::{get, post}, Router};
use tower_http::{trace::TraceLayer, cors::{CorsLayer, Any}};
use tracing_subscriber::EnvFilter;

mod state;
mod routes;
mod auth;
mod errors;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let state = AppState::new_from_env().await?;

    let app = Router::new()
        .route("/health", get(routes::health))
        .nest("/v1", routes::router())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
