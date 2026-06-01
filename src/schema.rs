// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
        ingredients -> Array<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
    }
}

diesel::table! {
    users_favorite_recipes (user_id, recipe_id) {
        user_id -> Int4,
        recipe_id -> Int4,
    }
}

diesel::joinable!(users_favorite_recipes -> recipes (recipe_id));
diesel::joinable!(users_favorite_recipes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(recipes, users, users_favorite_recipes,);
