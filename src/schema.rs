// @generated automatically by Diesel CLI.

diesel::table! {
    edges (start, finish) {
        start -> Integer,
        finish -> Integer,
        payload -> Text,
    }
}

diesel::table! {
    vertices (id) {
        id -> Integer,
        payload -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    edges,
    vertices,
);
