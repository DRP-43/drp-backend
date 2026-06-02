use crate::api::AppState;
use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router(openapi: OpenApi) -> (Router<AppState>, OpenApi) {
    OpenApiRouter::with_openapi(openapi)
        .nest("/v1", OpenApiRouter::default().routes(routes!(hello)))
        .split_for_parts()
}

#[utoipa::path(get, path = "/hello")]
pub async fn hello() -> &'static str {
    "Hello, World!"
}
