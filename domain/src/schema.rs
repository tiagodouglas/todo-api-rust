// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        description -> Varchar,
        completed -> Bool,
        userid -> Int4,
        datecreated -> Timestamptz,
        dateupdated -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        hash -> Text,
        datecreated -> Timestamptz,
        dateupdated -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(todos -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
