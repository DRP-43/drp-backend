use serde::{Deserialize, Serialize};

#[cfg(feature = "db")]
use diesel::prelude::*;

#[cfg(feature = "api")]
use utoipa::ToSchema;

pub type UserId = i64;

/// A user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable,)
)]
#[cfg_attr(feature = "api", derive(ToSchema))]
#[cfg_attr(feature="db", diesel(table_name = crate::schema::users))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct User {
    /// The ID for the user.
    pub id: UserId,

    /// Whether or not this is a development/testing user (FOR DEVELEOPMENT ONLY)
    #[cfg(debug_assertions)]
    pub __is_dev_: bool,
}

pub type RecipeId = i64;

/// A recipe that a user has favorited
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable,)
)]
#[cfg_attr(feature = "api", derive(ToSchema))]
#[cfg_attr(feature="db", diesel(table_name = crate::schema::recipes))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Recipe {
    /// The ID for the recipe.
    pub id: RecipeId,

    /// The name of the recipe.
    pub name: String,

    /// The ingredients in the recipe.
    pub ingredients: Vec<String>,
}

/// A recipe favorited by a user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable, Associations)
)]
#[cfg_attr(feature="db", diesel(table_name = crate::schema::users_favorite_recipes))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "db", diesel(belongs_to(User)))]
#[cfg_attr(feature = "db", diesel(belongs_to(Recipe)))]
#[cfg_attr(feature = "db", diesel(primary_key(user_id, recipe_id)))]
pub struct UserFavoritedRecipe {
    /// The ID for the user.
    pub user_id: UserId,

    /// The ID for the recipe.
    pub recipe_id: RecipeId,
}

/// A recipe queued by a user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable, Associations)
)]
#[cfg_attr(feature="db", diesel(table_name = crate::schema::users_queued_recipes))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "db", diesel(belongs_to(User)))]
#[cfg_attr(feature = "db", diesel(belongs_to(Recipe)))]
#[cfg_attr(feature = "db", diesel(primary_key(user_id, recipe_id)))]
pub struct UserQueuedRecipe {
    /// The ID for the user.
    pub user_id: UserId,

    /// The ID for the recipe.
    pub recipe_id: RecipeId,

    /// The queue number for the recipe.
    pub queue_number: i32,
}

/// A review for a recipe by a user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable, Associations)
)]
#[cfg_attr(feature="db", diesel(table_name = crate::schema::recipe_reviews))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "db", diesel(belongs_to(User)))]
#[cfg_attr(feature = "db", diesel(belongs_to(Recipe)))]
#[cfg_attr(feature = "db", diesel(primary_key(user_id, recipe_id)))]
pub struct RecipeReview {
    /// The ID for the user.
    pub user_id: UserId,

    /// The ID for the recipe.
    pub recipe_id: RecipeId,

    /// The rating the user gave the recipe.
    pub rating: i16,
}
