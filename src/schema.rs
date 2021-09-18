table! {
    flyway_schema_history (installed_rank) {
        installed_rank -> Int4,
        version -> Nullable<Varchar>,
        description -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        script -> Varchar,
        checksum -> Nullable<Int4>,
        installed_by -> Varchar,
        installed_on -> Timestamp,
        execution_time -> Int4,
        success -> Bool,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        avatar_url -> Nullable<Varchar>,
        quot -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    flyway_schema_history,
    users,
);
