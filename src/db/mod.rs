pub(crate) mod models;
pub(crate) mod schema;

use self::models::*;
use self::schema::*;
use crate::errors::*;
use diesel::prelude::*;

/// Establish a database connection.
pub fn establish_connection(database_url: String) -> Result<PgConnection> {
    let conn = PgConnection::establish(&database_url)?;
    Ok(conn)
}

/// Stupid stuff test
pub fn __test(conn: &mut PgConnection) -> Result<Vec<Recipe>> {
    let res = recipes::table.load::<Recipe>(conn)?;
    Ok(res)
}

/// Checks if a user is authenticated. Returns an error if it is. If not, returns the user database
/// entry.
pub fn check_user_authenticated(conn: &mut PgConnection) -> Result<User> {
    todo!()
}

/// Gets the user's favorited recipes. This has no specific order.
pub fn get_favorited_recipes(conn: &mut PgConnection, user_id: UserId) -> Result<Vec<Recipe>> {
    let user = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .get_result(conn)?;

    let queued_recipes = UserFavoritedRecipe::belonging_to(&user)
        .inner_join(recipes::table)
        .select(Recipe::as_select())
        .load(conn)?;

    Ok(queued_recipes)
}

/// Gets the user's queued recipes, in ascending order (i.e. starts with recipe 0, then 1, etc.).
pub fn get_queued_recipes(conn: &mut PgConnection, user_id: UserId) -> Result<Vec<Recipe>> {
    let user = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .get_result(conn)?;

    let queued_recipes = UserQueuedRecipe::belonging_to(&user)
        .inner_join(recipes::table)
        .select(Recipe::as_select())
        .order_by(users_queued_recipes::queue_number.asc())
        .load(conn)?;

    Ok(queued_recipes)
}
