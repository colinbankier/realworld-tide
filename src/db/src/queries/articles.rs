use crate::models::{Article, NewArticle, UpdateArticle, User};
use crate::queries::favorites::n_favorites;
use crate::schema::articles;
use crate::shims::to_article;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_query;
use domain;
use domain::ArticleQuery;
use std::collections::HashSet;
use std::iter::FromIterator;
use uuid::Uuid;

pub fn insert(repo: &Repo, article: NewArticle) -> Result<Article, Error> {
    diesel::insert_into(articles::table)
        .values(&article)
        .get_result(&repo.conn())
}

pub fn update(
    repo: &Repo,
    article_update: UpdateArticle,
    slug_value: &str,
) -> Result<Article, Error> {
    use crate::schema::articles::dsl::{articles, slug};

    diesel::update(articles.filter(slug.eq(slug_value)))
        .set(&article_update)
        .get_result(&repo.conn())
}

pub fn delete(repo: &Repo, slug_value: &str) -> Result<(), Error> {
    use crate::schema::articles::dsl::{articles, slug};

    diesel::delete(articles.filter(slug.eq(slug_value)))
        .execute(&repo.conn())
        // Discard the number of deleted rows
        .map(|_| ())
}

pub fn find(repo: &Repo, query: ArticleQuery) -> Result<Vec<(Article, User, u64)>, Error> {
    use crate::schema::articles::dsl::*;
    use crate::schema::users::dsl::{username, users};

    let results: Vec<(Article, User)> = {
        let q = articles
            .inner_join(users)
            .select((articles::all_columns(), users::all_columns()))
            .into_boxed();

        let q = if let Some(a) = query.author {
            q.filter(username.eq(a))
        } else {
            q
        };

        q.load(&repo.conn())
    }?;
    results
        .into_iter()
        .map(|(article, user)| {
            n_favorites(&repo, &article.slug).map(|n_fav| (article, user, n_fav as u64))
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn find_one(repo: &Repo, slug_value: &str) -> Result<domain::Article, Error> {
    use crate::schema::articles::dsl::{articles, slug};
    use crate::schema::users::dsl::users;

    let (article, user): (Article, User) = articles
        .filter(slug.eq(slug_value))
        .inner_join(users)
        .select((articles::all_columns(), users::all_columns()))
        .first(&repo.conn())?;
    let n_fav = n_favorites(&repo, &article.slug)?;
    let article = to_article(article, user.into(), n_fav as u64);
    Ok(article)
}

pub fn feed(
    repo: &Repo,
    user_id_value: Uuid,
    limit: u64,
    offset: u64,
) -> Result<Vec<(Article, User, u64)>, Error> {
    use crate::schema::articles::dsl::{articles, created_at, user_id};
    use crate::schema::followers::dsl::{followed_id, follower_id, followers};
    use crate::schema::users::dsl::{id, users};

    let limit = limit as i64;
    let offset = offset as i64;

    let results: Vec<(Article, User)> = followers
        .filter(follower_id.eq(user_id_value))
        .inner_join(users.on(id.eq(followed_id)))
        .inner_join(articles.on(user_id.eq(id)))
        .select((articles::all_columns(), users::all_columns()))
        .order(created_at.desc())
        .limit(limit)
        .offset(offset)
        .get_results(&repo.conn())?;
    results
        .into_iter()
        .map(|(article, user)| {
            n_favorites(&repo, &article.slug).map(|n_fav| (article, user, n_fav as u64))
        })
        .collect::<Result<Vec<_>, _>>()
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
    let mut result: Vec<Tags> = query.load(&repo.conn())?;
    // This is not actually an array: it's either a single element of empty
    let tags = match result.pop() {
        Some(tags) => HashSet::from_iter(tags.tags.into_iter()),
        None => HashSet::new(),
    };
    Ok(tags)
}
