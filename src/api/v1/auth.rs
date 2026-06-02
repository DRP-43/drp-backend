use crate::errors::*;
use crate::{api::AppState, errors, models::User, schema::users};
use axum::{
    extract::{Request, State},
    http,
    middleware::Next,
    response::Response,
};
use diesel::prelude::*;

/// Middleware to authorize users' requests.
pub async fn auth(State(state): State<AppState>, mut req: Request, next: Next) -> Result<Response> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(Error::AuthNoToken);
    };

    let user = state.query_db(|db_conn| authorize_current_user(db_conn, auth_header))?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

fn authorize_current_user(conn: &mut PgConnection, auth_token: &str) -> errors::Result<User> {
    println!("{auth_token}");

    // TODO: Authorize like this:
    //  1. Get device id from auth token
    //  2. check device id hash matches db entry for a user
    //  3. get user that matches
    //  4. if no match, unauthorized!
    //
    // TODO: Move into `db::` or refactor entire codebase
    let user_id = uuid::Uuid::nil();

    let user = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .get_result(conn)?;

    Ok(user)
}
