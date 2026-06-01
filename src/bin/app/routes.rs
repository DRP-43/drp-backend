use crate::state::AppState;
use axum::{Router, extract::State, http::StatusCode, routing::get};
use drp_backend::db::__test;
use std::sync::Arc;

/// Get the root-level routing of the app.
pub fn routes<S>(app_state: AppState) -> Router<S>
where
    S: Sync + Send + Clone + 'static,
{
    // Build our application with a route
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `health`
        .route("/health", get(health))
        // Add the state!
        .with_state(Arc::new(app_state))
}

/// basic handler that responds with a static string
async fn root(State(state): State<Arc<AppState>>) -> String {
    format!("{:?}", __test(&mut state.db_conn.lock().unwrap()).unwrap())
}

/// Health-check path
async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "service is online!")
}
