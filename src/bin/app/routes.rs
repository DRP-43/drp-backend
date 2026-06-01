use axum::{Router, http::StatusCode, routing::get};

/// Get the root-level routing of the app.
pub fn routes<S>() -> Router<S>
where
    S: Sync + Send + Clone + 'static,
{
    // Build our application with a route
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `health`
        .route("/health", get(health))
}

/// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

/// Health-check path
async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "service is online!")
}
