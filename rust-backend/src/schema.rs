// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        color -> Nullable<Varchar>,
        icon -> Nullable<Varchar>,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        amount -> Numeric,
        date -> Timestamptz,
        merchant -> Varchar,
        category_id -> Nullable<Uuid>,
        notes -> Nullable<Text>,
        items -> Nullable<Jsonb>,
        image_path -> Nullable<Varchar>,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    users,
    categories,
    transactions,
); 