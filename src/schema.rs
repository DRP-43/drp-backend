// @generated automatically by Diesel CLI.

diesel::table! {
    recipe_reviews (user_id, recipe_id) {
        user_id -> Int8,
        recipe_id -> Int8,
        rating -> Int2,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int8,
        name -> Varchar,
        body -> Text,
        ingredients -> Array<Jsonb>,
    }
}

diesel::table! {
    shopping_list (user_id, name) {
        user_id -> Int8,
        #[max_length = 128]
        name -> Varchar,
        quantity -> Float8,
        #[max_length = 8]
        unit -> Varchar,
        #[max_length = 8]
        category_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[sql_name = "__is_dev"]
        __is_dev_ -> Bool,
    }
}

diesel::table! {
    users_favorite_recipes (user_id, recipe_id) {
        user_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    users_inventory (user_id, name) {
        user_id -> Int8,
        #[max_length = 128]
        name -> Varchar,
        quantity -> Float8,
        #[max_length = 8]
        unit -> Varchar,
        #[max_length = 8]
        category_id -> Varchar,
    }
}

diesel::table! {
    users_queued_recipes (user_id, recipe_id) {
        user_id -> Int8,
        recipe_id -> Int8,
        queue_number -> Int4,
    }
}

diesel::joinable!(recipe_reviews -> recipes (recipe_id));
diesel::joinable!(recipe_reviews -> users (user_id));
diesel::joinable!(shopping_list -> users (user_id));
diesel::joinable!(users_favorite_recipes -> recipes (recipe_id));
diesel::joinable!(users_favorite_recipes -> users (user_id));
diesel::joinable!(users_inventory -> users (user_id));
diesel::joinable!(users_queued_recipes -> recipes (recipe_id));
diesel::joinable!(users_queued_recipes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    recipe_reviews,
    recipes,
    shopping_list,
    users,
    users_favorite_recipes,
    users_inventory,
    users_queued_recipes,
);
