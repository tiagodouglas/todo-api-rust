// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        hash -> Text,
        datecreated -> Timestamptz,
        dateupdated -> Nullable<Timestamptz>,
    }
}
