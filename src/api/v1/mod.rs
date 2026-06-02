mod auth;

use crate::api::AppState;
use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router(openapi: OpenApi, state: AppState) -> (Router<AppState>, OpenApi) {
    OpenApiRouter::<AppState>::with_openapi(openapi)
        .nest(
            "/v1",
            OpenApiRouter::default()
                .routes(routes!(hello))
                .nest("/user", user::router(state)),
        )
        .split_for_parts()
}

#[utoipa::path(get, path = "/hello")]
async fn hello() -> &'static str {
    "Hello, World!"
}

// User API functions
mod user {
    use super::auth;
    use crate::{api::AppState, db::models::User};
    use axum::{Extension, Json, middleware};
    use utoipa_axum::{router::OpenApiRouter, routes};

    pub fn router(state: AppState) -> OpenApiRouter<AppState> {
        OpenApiRouter::<AppState>::default()
            .routes(routes!(user))
            .route_layer(middleware::from_fn_with_state(state, auth::auth))
    }

    /// Get the user's information.
    #[utoipa::path(
        get,
        path = "/",
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user"),
            (status = OK, description = "The user's information", body = User)
        )
    )]
    #[axum::debug_handler]
    async fn user(Extension(user): Extension<User>) -> Json<User> {
        Json::from(user)
    }
}
