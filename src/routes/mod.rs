use axum::Router;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{fred::prelude::RedisPool, RedisStore};
use crate::state::AppState;

mod admin;
mod api;
mod public;

pub fn app(
    state: AppState,
    session_layer: SessionManagerLayer<RedisStore<RedisPool>>,
) -> Router {
    let base_path = state.base_path.clone();

    Router::new()
        .merge(public::routes(&base_path))
        .nest(&base_path, admin::routes())
        .nest("/api", api::routes())
        .nest_service("/static", ServeDir::new("static"))
        .fallback(crate::controllers::page_controller::handle_404)
        .layer(TraceLayer::new_for_http())
        .layer(session_layer)
        .with_state(state)
}
