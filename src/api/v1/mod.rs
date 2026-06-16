mod middlewares;
mod quotes;
mod recipe;
mod user;

use crate::api::AppState;
use axum::Router;
use utoipa::{OpenApi, openapi};
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "DRP43 backend API. NOTE: BOTH AUTHORIZATION METHODS MUST BE USED IN ORDER FOR API ACCESS TO WORK! DO NOT LISTEN WHEN IT ACCEPTS ONLY ONE FILLED IN! BOTH MUST BE FILLED IN!",
    ),
    nest(
         (path = "/v1/user", api = user::UserApiDoc),
         (path = "/v1/recipe", api = recipe::RecipeApiDoc),
         (path = "/v1/quotes", api = quotes::QuotesApiDoc),
     )
)]
pub struct V1ApiDoc;

pub fn router(state: AppState) -> (Router<AppState>, openapi::OpenApi) {
    OpenApiRouter::<AppState>::with_openapi(V1ApiDoc::openapi())
        .nest("/v1/user", user::router(state.clone()))
        .nest("/v1/recipe", recipe::router(state.clone()))
        .nest("/v1/quotes", quotes::router(state))
        .split_for_parts()
}
