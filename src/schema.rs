table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        description -> Varchar,
        body -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        token -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(articles -> users (user_id));

allow_tables_to_appear_in_same_query!(articles, users,);
