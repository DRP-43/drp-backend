// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        ingredients -> Array<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
    }
}

diesel::joinable!(recipes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(recipes, users,);
