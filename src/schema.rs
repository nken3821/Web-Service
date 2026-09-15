// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 25]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 30]
        email -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
