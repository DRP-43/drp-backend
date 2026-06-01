use diesel::prelude::*;

/// A user
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// The ID for the user.
    pub id: i32,
}

/// A recipe that a user has favorited
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    /// The ID for the recipe.
    pub id: i32,

    /// The user which owns the recipe.
    pub user_id: i32,

    /// The name of the recipe.
    pub name: String,

    /// The ingredients in the recipe.
    pub ingredients: Vec<String>,
}
