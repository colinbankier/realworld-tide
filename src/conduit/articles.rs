use crate::db;
use crate::models::{Article, NewArticle};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Deserialize;
use std::str::FromStr;

use crate::schema::articles;

type Repo = db::Repo<PgConnection>;

// joinable!(articles -> users (user_id));

#[derive(Default, Deserialize, Debug)]
pub struct ArticleQuery {
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub tag: Option<String>,
}

impl FromStr for ArticleQuery {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_urlencoded::from_str::<ArticleQuery>(s).map_err(|e| e.to_string())
    }
}

#[allow(dead_code)]
pub fn insert(repo: &Repo, article: NewArticle) -> Result<Article, Error> {
    repo.run(move |conn| {
        diesel::insert_into(articles::table)
            .values(&article)
            .get_result(&conn)
    })
}

pub fn find(repo: &Repo, query: ArticleQuery) -> Result<Vec<Article>, Error> {
    use crate::schema::articles::dsl::*;
    use crate::schema::users::dsl::{username, users};

    repo.run(move |conn| {
        let q = users
            .inner_join(articles)
            .select(articles::all_columns())
            .into_boxed();

        let q = if let Some(a) = query.author {
            q.filter(username.eq(a))
        } else {
            q
        };

        q.load(&conn)
    })
}

pub fn find_one(repo: &Repo, slug_value: &str) -> Result<Article, Error> {
    use crate::schema::articles::dsl::{articles, slug};
    use crate::schema::users::dsl::users;

    let slug_value = slug_value.to_owned();
    repo.run(move |conn| {
        articles
            .filter(slug.eq(slug_value))
            .inner_join(users)
            .select(articles::all_columns())
            .first(&conn)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conduit::users;
    use crate::models::{NewArticle, NewUser};
    use crate::test_helpers::test_server::get_repo;

    use crate::auth::encode_token;
    use futures_executor::ThreadPool;
    use uuid::Uuid;
    // use tokio_async_await_test::async_test;

    // #[async_test]
    // async fn test_list_articles() {
    //     let repo = Repo::new();

    //     let users =  create_users(&repo, 5).await ;
    //     let _articles =  create_articles(&repo, users);
    //     let results =
    //         find(repo.clone(), Default::default()).await.expect("Failed to get articles");

    //     assert_eq!(results.len(), 5);
    // }

    #[test]
    fn insert_and_retrieve_article() {
        let runtime = ThreadPool::new().unwrap();
        runtime.spawn_ok(async move {
            let repo = get_repo();
            let slug = "my_slug".to_string();

            let user_id = Uuid::new_v4();
            let token = encode_token(user_id);
            let user = NewUser {
                username: "my_user".into(),
                email: "my_email@hotmail.com".into(),
                password: "somepass".into(),
                id: user_id,
                token,
            };
            let user = users::insert(&repo, user).unwrap();

            let article = NewArticle {
                title: "My article".into(),
                slug: slug.clone(),
                description: "My article description".into(),
                body: "ohoh".into(),
                user_id: user.id,
            };
            let expected_article = insert(&repo, article).unwrap();

            let retrieved_article = find_one(&repo, &slug).unwrap();
            assert_eq!(expected_article, retrieved_article);
        })
    }
}
