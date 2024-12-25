// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (token) {
        #[max_length = 255]
        token -> Varchar,
        #[max_length = 255]
        hostname -> Varchar,
        #[max_length = 255]
        ip_address -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}
