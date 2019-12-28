use crate::conduit::favorites::n_favorites;
use crate::db::models::{Article, NewArticle, UpdateArticle, User};
use crate::db::schema::articles;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_query;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Default, Serialize, Deserialize, Debug)]
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

pub fn update(
    repo: &Repo,
    article_update: UpdateArticle,
    slug_value: String,
) -> Result<Article, Error> {
    use crate::db::schema::articles::dsl::{articles, slug};

    repo.run(move |conn| {
        diesel::update(articles.filter(slug.eq(slug_value)))
            .set(&article_update)
            .get_result(&conn)
    })
}

pub fn delete(repo: &Repo, slug_value: String) -> Result<(), Error> {
    use crate::db::schema::articles::dsl::{articles, slug};

    let to_be_deleted = articles.filter(slug.eq(slug_value));
    repo.run(move |conn| {
        diesel::delete(to_be_deleted)
            .execute(&conn)
            // Discard the number of deleted rows
            .map(|_| ())
    })
}

pub fn find(repo: &Repo, query: ArticleQuery) -> Result<Vec<(Article, User, i64)>, Error> {
    use crate::db::schema::articles::dsl::*;
    use crate::db::schema::users::dsl::{username, users};

    let results: Vec<(Article, User)> = repo.run(move |conn| {
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
    })?;
    results
        .into_iter()
        .map(|(article, user)| n_favorites(&repo, article.id).map(|n_fav| (article, user, n_fav)))
        .collect::<Result<Vec<_>, _>>()
}

pub fn find_one(repo: &Repo, slug_value: &str) -> Result<(Article, User, i64), Error> {
    use crate::db::schema::articles::dsl::{articles, slug};
    use crate::db::schema::users::dsl::users;

    let slug_value = slug_value.to_owned();
    let (article, user): (Article, User) = repo.run(move |conn| {
        articles
            .filter(slug.eq(slug_value))
            .inner_join(users)
            .select((articles::all_columns(), users::all_columns()))
            .first(&conn)
    })?;
    let n_fav = n_favorites(&repo, article.id)?;
    Ok((article, user, n_fav))
}

/// Fetching ALL tags seems like a really bad idea in a proper application.
pub fn tags(repo: &Repo) -> Result<HashSet<String>, Error> {
    use diesel::pg::types::sql_types::Array;
    use diesel::sql_types::Text;

    #[derive(QueryableByName)]
    pub struct Tags {
        #[sql_type = "Array<Text>"]
        pub tags: Vec<String>,
    }

    let query = sql_query("SELECT array_agg(DISTINCT tag) as tags FROM (SELECT 1, unnest(tag_list) FROM articles) AS t(id, tag) GROUP BY id");
    let mut result: Vec<Tags> = repo.run(move |conn| query.load(&conn))?;
    // This is not actually an array: it's either a single element of empty
    let tags = match result.pop() {
        Some(tags) => HashSet::from_iter(tags.tags.into_iter()),
        None => HashSet::new(),
    };
    Ok(tags)
}
