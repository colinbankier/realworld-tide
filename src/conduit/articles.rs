use crate::db;
use crate::db::models::{Article, NewArticle, User};
use crate::db::schema::articles;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Deserialize;
use std::str::FromStr;

type Repo = db::Repo<PgConnection>;

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

pub fn insert(repo: &Repo, article: NewArticle) -> Result<Article, Error> {
    repo.run(move |conn| {
        diesel::insert_into(articles::table)
            .values(&article)
            .get_result(&conn)
    })
}

pub fn find(repo: &Repo, query: ArticleQuery) -> Result<Vec<(Article, User)>, Error> {
    use crate::db::schema::articles::dsl::*;
    use crate::db::schema::users::dsl::{username, users};

    repo.run(move |conn| {
        let q = articles
            .inner_join(users)
            .select((articles::all_columns(), users::all_columns()))
            .into_boxed();

        let q = if let Some(a) = query.author {
            q.filter(username.eq(a))
        } else {
            q
        };

        q.load(&conn)
    })
}

pub fn find_one(repo: &Repo, slug_value: &str) -> Result<(Article, User), Error> {
    use crate::db::schema::articles::dsl::{articles, slug};
    use crate::db::schema::users::dsl::users;

    let slug_value = slug_value.to_owned();
    repo.run(move |conn| {
        articles
            .filter(slug.eq(slug_value))
            .inner_join(users)
            .select((articles::all_columns(), users::all_columns()))
            .first(&conn)
    })
}
