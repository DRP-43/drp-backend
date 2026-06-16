use crate::models::*;
use crate::schema::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

/// gets a recipe's statistics
pub fn get_recipe_stats(conn: &mut PgConnection, id: RecipeId) -> QueryResult<(f64, i64)> {
    let (rating, num_reviews) = recipe_reviews::table
        .filter(recipe_reviews::recipe_id.eq(id))
        .select((avg(recipe_reviews::rating), count(recipe_reviews::rating)))
        .get_result::<(Option<BigDecimal>, i64)>(conn)?;

    let rating = rating.unwrap_or_default().to_f64().unwrap_or_default();
    Ok((rating, num_reviews))
}

/// gets many recipes' statistics
pub fn get_many_recipes_stats(
    conn: &mut PgConnection,
    ids: Vec<RecipeId>,
) -> QueryResult<Vec<(f64, i64)>> {
    let results: Vec<(Option<BigDecimal>, i64)> = recipe_reviews::table
        .filter(recipe_reviews::recipe_id.eq_any(&ids))
        .group_by(recipe_reviews::recipe_id)
        .select((avg(recipe_reviews::rating), count(recipe_reviews::rating)))
        .get_results(conn)?;

    Ok(results
        .into_iter()
        .map(|(opt_rating, num_reviews)| {
            (
                opt_rating.unwrap_or_default().to_f64().unwrap_or_default(),
                num_reviews,
            )
        })
        .collect())
}

/// Gets a recipe by its ID.
pub fn get_recipe_from_id(conn: &mut PgConnection, id: RecipeId) -> QueryResult<Recipe> {
    let row = recipes::table
        .filter(recipes::id.eq(id))
        .select(RecipeRow::as_select())
        .first::<RecipeRow>(conn)?;

    get_recipe_from_row(conn, row)
}

/// Gets a recipe from a [`RecipeRow`] object.
pub(crate) fn get_recipe_from_row(conn: &mut PgConnection, row: RecipeRow) -> QueryResult<Recipe> {
    let ingredients = RecipeIngredientRow::belonging_to(&row).load::<RecipeIngredientRow>(conn)?;
    let (rating, num_reviews) = get_recipe_stats(conn, row.id)?;

    Ok(Recipe::from_row(row, ingredients, rating, num_reviews))
}

/// Gets multiple recipes from a `Vec` of [`RecipeRow`] objects. Preserves the order in which the
/// recipes are returned, i.e. the order of the `RecipeRow`s will be the order of the `Recipe`s.
pub(crate) fn get_recipes_from_rows(
    conn: &mut PgConnection,
    rows: Vec<RecipeRow>,
) -> QueryResult<Vec<Recipe>> {
    let ingredients = RecipeIngredientRow::belonging_to(&rows)
        .select(RecipeIngredientRow::as_select())
        .load::<RecipeIngredientRow>(conn)?
        .grouped_by(&rows);

    // Batch fetch all stats
    let ids: Vec<RecipeId> = rows.iter().map(|r| r.id).collect();
    let stats = get_many_recipes_stats(conn, ids)?;

    let recipes = rows
        .into_iter()
        .zip(ingredients)
        .zip(stats)
        .map(|((row, ingredients), (rating, num_reviews))| {
            Recipe::from_row(row, ingredients, rating, num_reviews)
        })
        .collect();

    Ok(recipes)
}
