use crate::api::AppState;
use crate::errors::*;
use crate::models::*;
use crate::schema::*;
use axum::extract::Path;
use axum::extract::Request;
use axum::extract::State;
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use diesel::prelude::*;

/// Middleware to get the recipe from the database when the path contains `{recipe_id}`.
pub async fn get_recipe(
    Path(recipe_id): Path<RecipeId>,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    let recipe = state.query_db(|conn| {
        recipes::table
            .filter(recipes::id.eq(recipe_id))
            .select(Recipe::as_select())
            .get_result(conn)
    })?;

    req.extensions_mut().insert(recipe);

    Ok(next.run(req).await)
}

/// Middleware to authorize users' requests, and get the user when the path contains `{user_id}`.
pub async fn auth_get_user(
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
