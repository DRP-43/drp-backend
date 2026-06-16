use crate::models::*;
use crate::schema::*;
use diesel::BelongingToDsl;
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

/// Gets a recipe by its ID.
pub fn get_recipe_from_id(conn: &mut PgConnection, id: RecipeId) -> QueryResult<Recipe> {
    let row = recipes::table
        .filter(recipes::id.eq(id))
        .select(RecipeRow::as_select())
        .first::<RecipeRow>(conn)?;

    get_recipe_from_row(conn, row)
}

/// Gets a recipe from a [`RecipeRow`] object.
pub fn get_recipe_from_row(conn: &mut PgConnection, row: RecipeRow) -> QueryResult<Recipe> {
    let ingredients = RecipeIngredientRow::belonging_to(&row).load::<RecipeIngredientRow>(conn)?;

    Ok(Recipe::from_row(row, ingredients))
}

/// Gets multiple recipes from a `Vec` of [`RecipeRow`] objects. Preserves the order in which the
/// recipes are returned, i.e. the order of the `RecipeRow`s will be the order of the `Recipe`s.
pub fn get_recipes_from_rows(
    conn: &mut PgConnection,
    rows: Vec<RecipeRow>,
) -> QueryResult<Vec<Recipe>> {
    let ingredients = RecipeIngredientRow::belonging_to(&rows)
        .select(RecipeIngredientRow::as_select())
        .load::<RecipeIngredientRow>(conn)?
        .grouped_by(&rows);

    let recipes = rows
        .into_iter()
        .zip(ingredients)
        .map(|(row, ingredients)| Recipe::from_row(row, ingredients))
        .collect();

    Ok(recipes)
}
