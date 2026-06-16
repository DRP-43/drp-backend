use crate::api::AppState;
use crate::db;
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
    let recipe = state.query_db(|conn| db::get_recipe_from_id(conn, recipe_id))?;

    req.extensions_mut().insert(recipe);

    Ok(next.run(req).await)
}

/// Middleware to authorize users' requests, and get the user when the path contains `{user_id}`.
pub async fn auth_get_user(
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

    // TODO: Fugly!
    let user_id: UserId = req
        .headers()
        .get("User")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<UserId>().ok())
        .unwrap_or(-1);

    let user: User = state.query_db(|conn| {
        authorize(conn, user_id, auth_header)?;

        let user = users::table
            .filter(users::id.eq(user_id))
            .select(User::as_select())
            .get_result(conn)?;

        Ok::<User, Error>(user)
    })?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

/// Authorize a user. `auth_header` is the field of the `Authorization` header. Errors if the user
/// isn't authed.
fn authorize(_conn: &mut PgConnection, _user_id: UserId, _auth_header: &str) -> Result<()> {
    // TODO: Authorize like this:
    //  1. Get device id from auth token
    //  2. check device id hash matches db entry for a user
    //  3. check user id matches device id user
    //  4. if match, authorized! if no match, unauthorized!
    Ok(())
}
