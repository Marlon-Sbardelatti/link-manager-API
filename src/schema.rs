// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Integer,
        url -> Text,
        created_at -> Timestamp,
    }
}
