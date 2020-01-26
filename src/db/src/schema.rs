table! {
    articles (slug) {
        title -> Varchar,
        slug -> Varchar,
        description -> Varchar,
        body -> Text,
        tag_list -> Array<Text>,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    comments (id) {
        id -> Int8,
        author_id -> Uuid,
        article_id -> Varchar,
        body -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    favorites (user_id, article_id) {
        user_id -> Uuid,
        article_id -> Varchar,
    }
}

table! {
    followers (followed_id, follower_id) {
        followed_id -> Uuid,
        follower_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(articles -> users (user_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (author_id));
joinable!(favorites -> articles (article_id));
joinable!(favorites -> users (user_id));

allow_tables_to_appear_in_same_query!(articles, comments, favorites, followers, users,);
