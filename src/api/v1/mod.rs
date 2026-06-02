mod auth;

use crate::api::AppState;
use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub fn router(openapi: OpenApi, state: AppState) -> (Router<AppState>, OpenApi) {
    OpenApiRouter::<AppState>::with_openapi(openapi)
        .nest("/v1/user", user::router(state))
        .split_for_parts()
}

// User API functions
mod user {
    use super::auth;
    use crate::api::AppState;
    use crate::errors::*;
    use crate::models::*;
    use crate::schema::*;
    use axum::extract::State;
    use axum::{Extension, Json, middleware};
    use diesel::prelude::*;
    use utoipa_axum::{router::OpenApiRouter, routes};

    pub fn router(state: AppState) -> OpenApiRouter<AppState> {
        OpenApiRouter::<AppState>::default()
            .routes(routes!(get_user))
            .routes(routes!(get_favorites))
            .routes(routes!(get_queue))
            .route_layer(middleware::from_fn_with_state(state, auth::auth))
    }

    /// Get the user's information.
    #[utoipa::path(
        get,
        path = "/",
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's information", body = User)
        )
    )]
    #[axum::debug_handler]
    async fn get_user(Extension(user): Extension<User>) -> Json<User> {
        Json::from(user)
    }

    /// Get the user's favorite recipes
    #[utoipa::path(
        get,
        path = "/favorites",
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's favorited recipes", body = Vec<Recipe>)
        )
    )]
    #[axum::debug_handler]
    async fn get_favorites(
        Extension(user): Extension<User>,
        State(state): State<AppState>,
    ) -> Result<Json<Vec<Recipe>>> {
        let favorited_recipes = state.query_db(|conn| {
            UserFavoritedRecipe::belonging_to(&user)
                .inner_join(recipes::table)
                .select(Recipe::as_select())
                .load(conn)
        })?;

        Ok(Json(favorited_recipes))
    }

    /// Get the user's recipe queue
    #[utoipa::path(
        get,
        path = "/queue",
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's recipe queue, ascending", body = Vec<Recipe>)
        )
    )]
    #[axum::debug_handler]
    async fn get_queue(
        Extension(user): Extension<User>,
        State(state): State<AppState>,
    ) -> Result<Json<Vec<Recipe>>> {
        let queued_recipes = state.query_db(|conn| {
            UserQueuedRecipe::belonging_to(&user)
                .inner_join(recipes::table)
                .select(Recipe::as_select())
                .order_by(users_queued_recipes::queue_number.asc())
                .load(conn)
        })?;

        Ok(Json(queued_recipes))
    }
}
