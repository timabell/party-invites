// @generated automatically by Diesel CLI.

diesel::table! {
    invites (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
        status -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        phone -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    invites,
    users,
);
