mod v1;

use axum::{Router, http::StatusCode, routing::get};
use diesel::PgConnection;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::{timeout, trace};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    /// The connection to the database.
    pub db_conn: Arc<Mutex<PgConnection>>,
}

impl AppState {
    /// Creates a new app state
    pub fn new(db_conn: PgConnection) -> Self {
        Self {
            db_conn: Arc::new(Mutex::new(db_conn)),
        }
    }

    /// Runs a function that uses a mutable reference to the database connection.
    pub fn query_db<T>(&self, f: impl FnOnce(&mut PgConnection) -> T) -> T where {
        let db_conn = &mut self
            .db_conn
            .lock()
            .expect("This mutex shouldn't be poisoned!");

        f(db_conn)
    }
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/v1", api = v1::V1ApiDoc)
    )
)]
pub struct ApiDoc;

/// Get the top-level root router for the app.
pub fn router(state: AppState) -> Router {
    let (v1, docs) = v1::router(state.clone());

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs))
        .merge(v1)
        .route("/health", get(async || "Service is online!"))
        // Add default layers and such
        .layer(
            ServiceBuilder::new()
                .layer(trace::TraceLayer::new_for_http())
                .layer(timeout::TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(120),
                )),
        )
        .with_state(state)
}
