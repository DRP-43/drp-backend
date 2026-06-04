use crate::api::AppState;
use crate::errors::*;
use crate::models::*;
use crate::schema::*;
use axum::extract::{Path, Request, State};
use axum::http;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Extension, Json};
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
        .routes(routes!(get_favorites))
        .routes(routes!(get_queue))
        .route_layer(middleware::from_fn_with_state(state, user_auth))
}

/// Get the user's information.
#[utoipa::path(
        get,
        path = "/{user_id}",
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

/// Get the user's favorite recipes
#[utoipa::path(
        get,
        path = "/{user_id}/favorites",
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

/// Get the user's recipe queue
#[utoipa::path(
        get,
        path = "/{user_id}/queue",
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

/// Middleware to authorize users' requests.
pub async fn user_auth(
    Path(user_id): Path<UserId>,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(Error::AuthNoToken);
    };

    let user = state.query_db(|conn| {
        // TODO: Authorize like this:
        //  1. Get device id from auth token
        //  2. check device id hash matches db entry for a user
        //  3. check user id matches device id user
        //  4. if match, authorized! if no match, unauthorized!

        let _ = auth_header; // NOTE: only here so we get rid of unused variable warning above

        users::table
            .filter(users::id.eq(user_id))
            .select(User::as_select())
            .get_result(conn)
    })?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
