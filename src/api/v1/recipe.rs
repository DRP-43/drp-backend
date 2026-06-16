use super::middlewares;
use crate::api::AppState;
use crate::errors::*;
use crate::models::*;
use crate::schema::*;
use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::middleware;
use diesel::dsl::*;
use diesel::prelude::*;
use utoipa::OpenApi;
use utoipa_axum::router::UtoipaMethodRouterExt;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi()]
pub struct RecipeApiDoc;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::<AppState>::with_openapi(RecipeApiDoc::openapi())
        .routes(routes!(get_recipe))
        .routes(routes!(get_recipe_rating))
        .routes(routes!(get_recipe_num_reviews))
        .routes(routes!(post_publish_recipe, put_edit_recipe).layer(
            middleware::from_fn_with_state(state.clone(), middlewares::auth_get_user),
        ))
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
async fn get_recipe_rating(Extension(recipe): Extension<Recipe>) -> Result<Json<f64>> {
    Ok(Json(recipe.rating))
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
async fn get_recipe_num_reviews(Extension(recipe): Extension<Recipe>) -> Result<Json<i64>> {
    Ok(Json(recipe.num_reviews))
}

/// Publish a recipe. Automatically overwrites the submitted recipe ID, so the submitted recipe ID
/// may be 0 (or any integer). Automatically sets the owner ID to the submitting user, so it can
/// be anything (probably just the user's id). Any non-editable statistics are not used.
#[utoipa::path(
        post,
        path = "/publish/under_user/{user_id}",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = Recipe, content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "Recipe published", body = ())
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn post_publish_recipe(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(mut recipe): Json<Recipe>,
) -> Result<()> {
    recipe.owner_id = user.id;
    recipe.id = state
        .query_db(|conn| {
            recipes::table
                .select(max(recipes::id))
                .get_result::<Option<RecipeId>>(conn)
        })
        .map(|x| x.unwrap_or(0))?;

    let (recipe_row, ingredients) = recipe.to_row_and_ingredients();

    state.query_db(|conn| recipe_row.insert_into(recipes::table).execute(conn))?;
    state.query_db(|conn| {
        insert_into(recipe_ingredients::table)
            .values(&ingredients)
            .execute(conn)
    })?;

    Ok(())
}

/// Edits a recipe. Requires the recipe ID to be set to `{recipe_id}`. Requires the user ID to be
/// set to `{user_id}`. Requires authentication under the `{user_id}` user. Requires the
/// `{recipe_id}` recipe to exist. Ignores the recipe ID given in the request body. Any non-editable
/// statistics are not used.
#[utoipa::path(
        put,
        path = "/{recipe_id}/edit/under_user/{user_id}",
        params(
            ("recipe_id" = RecipeId, Path, description = "UUID of the recipe"),
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = Recipe, content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "Recipe published", body = ())
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn put_edit_recipe(
    Path(recipe_id): Path<RecipeId>,
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(mut recipe): Json<Recipe>,
) -> Result<()> {
    if state.query_db(|conn| {
        recipes::table
            .filter(recipes::id.eq(recipe_id))
            .execute(conn)
    })? == 0
    {
        return Err(Error::UserEditingNonExistantRecipe(recipe_id));
    } else if state.query_db(|conn| {
        recipes::table
            .filter(recipes::id.eq(recipe_id))
            .filter(recipes::owner_id.eq(user.id))
            .execute(conn)
    })? == 0
    {
        return Err(Error::UserEditingUnownedRecipe(recipe_id));
    }

    recipe.owner_id = user.id;
    recipe.id = state
        .query_db(|conn| {
            recipes::table
                .select(max(recipes::id))
                .get_result::<Option<RecipeId>>(conn)
        })
        .map(|x| x.unwrap_or(0))?;

    let (recipe_row, ingredients) = recipe.to_row_and_ingredients();

    state.query_db(|conn| update(recipes::table).set(&recipe_row).execute(conn))?;

    // TODO: Better process than deleting every ingredient -> adding them back?
    state.query_db(|conn| {
        delete(recipe_ingredients::table)
            .filter(recipe_ingredients::recipe_id.eq(recipe_row.id))
            .execute(conn)
    })?;
    state.query_db(|conn| {
        insert_into(recipe_ingredients::table)
            .values(ingredients)
            .execute(conn)
    })?;

    Ok(())
}
