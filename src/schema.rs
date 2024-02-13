// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Integer,
        url -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    links,
    users,
);
