use axum::Router;
use crate::state::AppState;

pub mod health;
pub mod auth_routes;
pub mod devices;
pub mod keys;
pub mod messages;
pub mod attachments;

pub async fn health() -> &'static str { "ok" }

pub fn router() -> Router<AppState> {
    let v1_api = Router::new()
        .merge(auth_routes::router())
        .merge(devices::router())
        .merge(keys::router())
        .merge(messages::router())
        .merge(attachments::router());

    Router::new()
        .route("/health", axum::routing::get(health))
        .nest("/v1", v1_api)
}
