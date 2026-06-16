use super::middlewares;
use crate::api::AppState;
use crate::errors::*;
use crate::models::*;
use crate::schema::*;
use axum::Extension;
use axum::Json;
use axum::extract::State;
use axum::middleware;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use diesel::dsl::*;
use diesel::prelude::*;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi()]
pub struct RecipeApiDoc;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::<AppState>::with_openapi(RecipeApiDoc::openapi())
        .routes(routes!(get_recipe))
        .routes(routes!(get_recipe_rating))
        .routes(routes!(get_recipe_num_reviews))
        .route_layer(middleware::from_fn_with_state(
            state,
            middlewares::get_recipe,
        ))
}

/// Get information for the `recipe_id` recipe.
#[utoipa::path(
        get,
        path = "/{recipe_id}",
        params(
            ("recipe_id" = RecipeId, Path, description = "UUID of the recipe")
        ),
        responses(
            (status = OK, description = "The recipe", body = Recipe)
        )
    )]
#[axum::debug_handler]
async fn get_recipe(Extension(recipe): Extension<Recipe>) -> Json<Recipe> {
    Json::from(recipe)
}

/// Get the rating of the `recipe_id` recipe.
#[utoipa::path(
        get,
        path = "/{recipe_id}/rating",
        params(
            ("recipe_id" = RecipeId, Path, description = "UUID of the recipe")
        ),
        responses(
            (status = OK, description = "The recipe", body = f64)
        )
    )]
#[axum::debug_handler]
async fn get_recipe_rating(
    State(state): State<AppState>,
    Extension(recipe): Extension<Recipe>,
) -> Result<Json<f64>> {
    let result = state
        .query_db(|conn| {
            recipe_reviews::table
                .filter(recipe_reviews::recipe_id.eq(recipe.id))
                .select(avg(recipe_reviews::rating))
                .get_result::<Option<BigDecimal>>(conn)
        })
        .map(Option::unwrap_or_default)
        .map(|x| x.to_f64())
        .map(Option::unwrap_or_default)
        .map(Json::from)?;

    Ok(result)
}

/// Get the number of reviews of the `recipe_id` recipe.
#[utoipa::path(
        get,
        path = "/{recipe_id}/num_reviews",
        params(
            ("recipe_id" = RecipeId, Path, description = "UUID of the recipe")
        ),
        responses(
            (status = OK, description = "The number of reviews for the recipe", body = i64)
        )
    )]
#[axum::debug_handler]
async fn get_recipe_num_reviews(
    State(state): State<AppState>,
    Extension(recipe): Extension<Recipe>,
) -> Result<Json<i64>> {
    let result = state
        .query_db(|conn| {
            recipe_reviews::table
                .filter(recipe_reviews::recipe_id.eq(recipe.id))
                .select(count(recipe_reviews::rating))
                .get_result::<i64>(conn)
        })
        .map(Json::from)?;

    Ok(result)
}
