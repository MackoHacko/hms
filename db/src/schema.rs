// @generated automatically by Diesel CLI.

diesel::table! {
    snips (id) {
        id -> Integer,
        alias -> Text,
        value -> Text,
        access_count -> Integer,
        created -> Timestamp,
        last_access -> Nullable<Timestamp>,
    }
}
