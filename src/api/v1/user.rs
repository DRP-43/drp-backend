use super::middlewares;
use crate::api::AppState;
use crate::errors::*;
use crate::models::*;
use crate::schema::*;
use axum::extract::State;
use axum::middleware;
use axum::{Extension, Json};
use diesel::dsl::*;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi(
    modifiers(&UserAuthModifier),
)]
pub struct UserApiDoc;

#[derive(Debug, Serialize)]
struct UserAuthModifier;

impl Modify for UserAuthModifier {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "user_device_id",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::<AppState>::with_openapi(UserApiDoc::openapi())
        .routes(routes!(get_user))
        .routes(routes!(get_inventory, post_inventory, delete_inventory))
        .routes(routes!(get_favorites, post_favorites, delete_favorites))
        .routes(routes!(get_favorites_ids))
        .routes(routes!(get_queue, post_queue, delete_queue))
        .route_layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_get_user,
        ))
}

/************************************************************************************************
 *                                         API ROUTES                                           *
 ************************************************************************************************/

/// Get the user's information.
#[utoipa::path(
        get,
        path = "/{user_id}",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's information", body = User)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn get_user(Extension(user): Extension<User>) -> Json<User> {
    Json::from(user)
}

/// Get the user's current inventory
#[utoipa::path(
        get,
        path = "/{user_id}/inventory",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's favorited recipes", body = Vec<Recipe>)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn get_inventory(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Ingredient>>> {
    let inventory = state.query_db(|conn| {
        users_inventory::table
            .filter(users_inventory::user_id.eq(user.id))
            // NOTE: Select is necessary so we strip the `user_id` component and can transform the
            // row into a valid `Ingredient`.
            .select((
                users_inventory::name,
                users_inventory::quantity,
                users_inventory::unit,
                users_inventory::category_id,
            ))
            .load::<Ingredient>(conn)
    })?;

    Ok(Json(inventory))
}

/// Add to the user's inventory
#[utoipa::path(
        post,
        path = "/{user_id}/inventory",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = Ingredient, content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's favorited recipes", body = Vec<Recipe>)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn post_inventory(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(ingredient): Json<Ingredient>,
) -> Result<Json<usize>> {
    let res = state.query_db(|conn| {
        insert_into(users_inventory::table)
            .values((&ingredient, users_inventory::user_id.eq(user.id)))
            .execute(conn)
    })?;

    Ok(Json(res))
}

/// Remove from the user's inventory
#[utoipa::path(
        delete,
        path = "/{user_id}/inventory",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = String, content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's favorited recipes", body = Vec<Recipe>)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn delete_inventory(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(ingredient_name): Json<String>,
) -> Result<Json<usize>> {
    let res = state.query_db(|conn| {
        delete(users_inventory::table)
            .filter(users_inventory::user_id.eq(user.id))
            .filter(users_inventory::name.eq(ingredient_name))
            .execute(conn)
    })?;

    Ok(Json(res))
}

/// Get the user's favorite recipes
#[utoipa::path(
        get,
        path = "/{user_id}/favorites",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's favorited recipes", body = Vec<Recipe>)
        ),
        security(
            ("user_device_id" = [])
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

/// Get the user's favorite recipes, but only the ids
#[utoipa::path(
        get,
        path = "/{user_id}/favorites/id",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's favorited recipes ids", body = Vec<UserId>)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn get_favorites_ids(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Vec<RecipeId>>> {
    let favorited_recipes = state.query_db(|conn| {
        UserFavoritedRecipe::belonging_to(&user)
            .select(users_favorite_recipes::recipe_id)
            .load(conn)
    })?;

    Ok(Json(favorited_recipes))
}

/// Add to the user's favorite recipes
#[utoipa::path(
        post,
        path = "/{user_id}/favorites",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = inline(RecipeId), content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "Recipe was added to the user's favorites", body = usize)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn post_favorites(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(recipe_id): Json<RecipeId>, // NOTE: This is just the body as a string parsed as a UUID
) -> Result<Json<usize>> {
    let res = state.query_db(|conn| {
        insert_into(users_favorite_recipes::table)
            .values(&UserFavoritedRecipe {
                user_id: user.id,
                recipe_id,
            })
            .execute(conn)
    })?;

    Ok(Json::from(res))
}

/// Removie a recipe from the user's favorite recipes
#[utoipa::path(
        delete,
        path = "/{user_id}/favorites",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = inline(RecipeId), content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "Recipe was deleted from the user's favorites", body = usize)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn delete_favorites(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(recipe_id): Json<RecipeId>, // NOTE: This is just the body as a string parsed as a UUID
) -> Result<Json<usize>> {
    let res = state.query_db(|conn| {
        delete(users_favorite_recipes::table)
            .filter(users_favorite_recipes::user_id.eq(user.id))
            .filter(users_favorite_recipes::recipe_id.eq(recipe_id))
            .execute(conn)
    })?;

    Ok(Json::from(res))
}

/// Get the user's recipe queue
#[utoipa::path(
        get,
        path = "/{user_id}/queue",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "The user's recipe queue, ascending", body = Vec<Recipe>)
        ),
        security(
            ("user_device_id" = [])
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

/// Add to the user's recipe queue. Puts it at the end of the queue (highest queue number).
#[utoipa::path(
        post,
        path = "/{user_id}/queue",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = inline(RecipeId), content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "Recipe was added to the user's queue", body = usize)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn post_queue(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(recipe_id): Json<RecipeId>, // NOTE: This is just the body as a string parsed as a UUID
) -> Result<Json<usize>> {
    // Get the queue number for this recipe. This is either:
    //  1. the maximum queue number for the user + 1, or (if it doesn't exist)
    //  2. 0 (if there are no queued recipes)
    let queue_number = state
        .query_db(|conn| {
            users_queued_recipes::table
                .filter(users_queued_recipes::user_id.eq(user.id))
                .select(max(users_queued_recipes::queue_number))
                .get_result::<Option<i32>>(conn)
        })?
        .map(|x| x + 1) // maximum number + 1 is the new queue number, or...
        .unwrap_or(0); // default to 0 if it doesn't exist

    let res = state.query_db(|conn| {
        insert_into(users_queued_recipes::table)
            .values(&UserQueuedRecipe {
                user_id: user.id,
                recipe_id,
                queue_number,
            })
            .execute(conn)
    })?;

    Ok(Json::from(res))
}

/// Remove from the user's recipe queue
#[utoipa::path(
        delete,
        path = "/{user_id}/queue",
        params(
            ("user_id" = UserId, Path, description = "UUID of the user")
        ),
        request_body(content = inline(RecipeId), content_type = "application/json"),
        responses(
            (status = UNAUTHORIZED, description = "Failed to authorize user", body = String),
            (status = OK, description = "Recipe was deleted from the user's queue", body = usize)
        ),
        security(
            ("user_device_id" = [])
        )
    )]
#[axum::debug_handler]
async fn delete_queue(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(recipe_id): Json<RecipeId>, // NOTE: This is just the body as a string parsed as a UUID
) -> Result<Json<usize>> {
    let res = state.query_db(|conn| {
        delete(users_queued_recipes::table)
            .filter(users_queued_recipes::user_id.eq(user.id))
            .filter(users_queued_recipes::recipe_id.eq(recipe_id))
            .execute(conn)
    })?;

    Ok(Json::from(res))
}
