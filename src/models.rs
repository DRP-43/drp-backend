use ::serde::Serialize;
use diesel::prelude::*;
use utoipa::ToSchema;
use uuid::*;

pub type UserId = Uuid;

/// A user
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, ToSchema, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// The ID for the user.
    #[schema(value_type = String, format = Uuid)]
    pub id: UserId,

    /// Whether or not this is a development/testing user (FOR DEVELEOPMENT ONLY)
    #[cfg(debug_assertions)]
    pub __is_dev_: bool,
}

pub type RecipeId = Uuid;

/// A recipe that a user has favorited
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, ToSchema, Serialize)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    /// The ID for the recipe.
    #[schema(value_type = String, format = Uuid)]
    pub id: RecipeId,

    /// The name of the recipe.
    pub name: String,

    /// The ingredients in the recipe.
    pub ingredients: Vec<String>,
}

/// A recipe favorited by a user.
#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = crate::schema::users_favorite_recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Recipe))]
#[diesel(primary_key(user_id, recipe_id))]
pub struct UserFavoritedRecipe {
    /// The ID for the user.
    pub user_id: UserId,

    /// The ID for the recipe.
    pub recipe_id: RecipeId,
}

/// A recipe queued by a user
#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = crate::schema::users_queued_recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Recipe))]
#[diesel(primary_key(user_id, recipe_id))]
pub struct UserQueuedRecipe {
    /// The ID for the user.
    pub user_id: UserId,

    /// The ID for the recipe.
    pub recipe_id: RecipeId,

    /// The queue number for the recipe.
    pub queue_number: i32,
}
