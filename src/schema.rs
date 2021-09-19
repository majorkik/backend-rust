table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        email -> Varchar,
        pass_hash -> Text,
        user_role -> Array<Text>,
    }
}
