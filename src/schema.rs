table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        avatar_url -> Nullable<Text>,
        quot -> Nullable<Text>,
    }
}
