// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Uuid,
        name -> Varchar,
        ingredients -> Array<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        __is_dev -> Bool,
    }
}

diesel::table! {
    users_favorite_recipes (user_id, recipe_id) {
        user_id -> Uuid,
        recipe_id -> Uuid,
    }
}

diesel::table! {
    users_queued_recipes (user_id, recipe_id) {
        user_id -> Uuid,
        recipe_id -> Uuid,
        queue_number -> Int4,
    }
}

diesel::joinable!(users_favorite_recipes -> recipes (recipe_id));
diesel::joinable!(users_favorite_recipes -> users (user_id));
diesel::joinable!(users_queued_recipes -> recipes (recipe_id));
diesel::joinable!(users_queued_recipes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    recipes,
    users,
    users_favorite_recipes,
    users_queued_recipes,
);
