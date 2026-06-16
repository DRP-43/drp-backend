use serde::{Deserialize, Serialize};

#[cfg(feature = "db")]
use ::{
    diesel::deserialize::FromSql,
    diesel::deserialize::FromSqlRow,
    diesel::expression::AsExpression,
    diesel::pg::Pg,
    diesel::prelude::*,
    diesel::serialize::{Output, ToSql},
    diesel::sql_types,
};

#[cfg(feature = "api")]
use ::utoipa::ToSchema;

pub type UserId = i64;

/// A user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable)
)]
#[cfg_attr(feature = "api", derive(ToSchema))]
#[cfg_attr(feature = "db", diesel(table_name = crate::schema::users))]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable)
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
    pub ingredients: Vec<Ingredient>,

    /// The body of the recipe, i.e. the "instructions"
    #[serde(rename = "recipe_body")]
    pub body: String,
}

/// An ingredient that a user has in their inventory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
#[cfg_attr(feature = "db", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "db", diesel(table_name = crate::schema::users_inventory))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
pub(crate) struct UserInventoryIngredientRow {
    /// The ID for the user.
    pub user_id: UserId,

    /// Name of the ingredient
    pub name: String,

    /// Quantity of the ingredient
    pub quantity: f64,

    /// The "unit", i.e. what measurement unit the ingredient has.
    pub unit: String,

    /// Expiration date of the ingredient
    #[serde(default)]
    pub expiration_date: Option<String>,

    /// What category of thing this belongs to.
    #[serde(default)]
    pub category_id: IngredientCategory,
}

/// An ingredient that a user has in their inventory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
#[cfg_attr(feature = "db", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "db", diesel(table_name = crate::schema::shopping_list))]
#[cfg_attr(feature = "db", diesel(check_for_backend(diesel::pg::Pg)))]
pub(crate) struct UserShoppingListRow {
    /// The ID for the user.
    pub user_id: UserId,

    /// Name of the ingredient
    pub name: String,

    /// Quantity of the ingredient
    pub quantity: f64,

    /// The "unit", i.e. what measurement unit the ingredient has.
    pub unit: String,

    /// What category of thing this belongs to.
    #[serde(default)]
    pub category_id: IngredientCategory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct Ingredient {
    /// Name of the ingredient
    pub name: String,

    /// Quantity of the ingredient
    pub quantity: f64,

    /// The "unit", i.e. what measurement unit the ingredient has.
    pub unit: String,

    /// Expiration date of the ingredient
    #[serde(default)]
    pub expiration_date: Option<String>,

    /// What category of thing this belongs to.
    #[serde(default)]
    pub category_id: IngredientCategory,
}

impl From<UserInventoryIngredientRow> for Ingredient {
    fn from(value: UserInventoryIngredientRow) -> Self {
        Self {
            name: value.name,
            quantity: value.quantity,
            unit: value.unit,
            expiration_date: value.expiration_date,
            category_id: value.category_id,
        }
    }
}

impl From<UserShoppingListRow> for Ingredient {
    fn from(value: UserShoppingListRow) -> Self {
        Self {
            name: value.name,
            quantity: value.quantity,
            unit: value.unit,
            expiration_date: None,
            category_id: value.category_id,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "api", derive(ToSchema))]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature="db", diesel(sql_type = sql_types::VarChar))]
pub enum IngredientCategory {
    #[serde(rename = "fruits")]
    Fruit,

    #[serde(rename = "veg")]
    Vegetables,

    Meat,

    #[serde(rename = "carbs")]
    Carb,

    #[serde(rename = "liquids")]
    Liquid,

    #[default]
    Other,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for IngredientCategory {
    fn to_string(&self) -> String {
        match self {
            IngredientCategory::Fruit => "fruits",
            IngredientCategory::Vegetables => "veg",
            IngredientCategory::Meat => "meat",
            IngredientCategory::Carb => "carbs",
            IngredientCategory::Liquid => "liquids",
            IngredientCategory::Other => "other",
        }
        .to_string()
    }
}

/// A quote for the price of an [`Ingredient`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(ToSchema))]
pub struct PriceQuote {
    /// The retailer where the quote came from
    pub retailer: String,

    /// The price (in pence) of the ingredient given.
    pub price_pence: u64,

    /// The URL to the product page.
    pub product_url: String,
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
    pub queue_number: i64,
}

/// A review for a recipe by a user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    derive(Queryable, Selectable, Identifiable, Insertable, Associations)
)]
#[cfg_attr(feature = "db", diesel(table_name = crate::schema::recipe_reviews))]
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

/****************
 * MANUAL IMPLS *
 ****************/

// NOTE: Need to impl manually so we can cast `Ingredient` to `sql_types::Jsonb` PgSQL types and v.v.
#[cfg(feature = "db")]
impl FromSql<sql_types::Jsonb, Pg> for Ingredient {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<sql_types::Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

// NOTE: Need to impl manually so we can cast `Ingredient` to `sql_types::Jsonb` PgSQL types and v.v.
#[cfg(feature = "db")]
impl ToSql<sql_types::Jsonb, Pg> for Ingredient {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<sql_types::Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "db")]
impl FromSql<sql_types::VarChar, Pg> for IngredientCategory {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let value = <String as FromSql<sql_types::VarChar, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_str(&value)?)
    }
}

#[cfg(feature = "db")]
impl ToSql<sql_types::VarChar, Pg> for IngredientCategory {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        let value = value.to_string();
        <String as ToSql<sql_types::VarChar, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}
