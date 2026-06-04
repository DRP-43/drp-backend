mod middlewares;
mod recipe;
mod user;

use crate::api::AppState;
use axum::Router;
use utoipa::{OpenApi, openapi};
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(
     nest(
         (path = "/v1/user", api = user::UserApiDoc),
         (path = "/v1/recipe", api = recipe::RecipeApiDoc),
     )
)]
pub struct V1ApiDoc;

pub fn router(state: AppState) -> (Router<AppState>, openapi::OpenApi) {
    OpenApiRouter::<AppState>::with_openapi(V1ApiDoc::openapi())
        .nest("/v1/user", user::router(state.clone()))
        .nest("/v1/recipe", recipe::router(state))
        .split_for_parts()
}
