// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        due_at -> Nullable<Timestamptz>,
        enabled -> Bool,
        user_email -> Nullable<Varchar>,
    }
}
