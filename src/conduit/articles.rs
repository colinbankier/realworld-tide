use crate::conduit::favorites::n_favorites;
use crate::db::models::{Article, NewArticle, UpdateArticle, User};
use crate::db::schema::articles;
use crate::domain;
use crate::domain::ArticleQuery;
use crate::domain::PublishArticleError;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::sql_query;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;
use uuid::Uuid;

impl FromStr for ArticleQuery {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_urlencoded::from_str::<ArticleQuery>(s).map_err(|e| e.to_string())
    }
}

pub fn insert(repo: &Repo, article: NewArticle) -> Result<Article, PublishArticleError> {
    repo.run(move |conn| {
        let result = diesel::insert_into(articles::table)
            .values(&article)
            .get_result(&conn);

        result.map_err(|e| match e {
            Error::DatabaseError(kind, _) => match kind {
                DatabaseErrorKind::UniqueViolation => PublishArticleError::DuplicatedSlug {
                    slug: article.slug,
                    source: e,
                },
                _ => PublishArticleError::DatabaseError(e),
            },
            e => PublishArticleError::DatabaseError(e),
        })
    })
}

pub fn update(
    repo: &Repo,
    article_update: UpdateArticle,
    slug_value: &str,
) -> Result<Article, Error> {
    use crate::db::schema::articles::dsl::{articles, slug};

    repo.run(move |conn| {
        diesel::update(articles.filter(slug.eq(slug_value)))
            .set(&article_update)
            .get_result(&conn)
    })
}

pub fn delete(repo: &Repo, slug_value: &str) -> Result<(), Error> {
    use crate::db::schema::articles::dsl::{articles, slug};

    repo.run(move |conn| {
        diesel::delete(articles.filter(slug.eq(slug_value)))
            .execute(&conn)
            // Discard the number of deleted rows
            .map(|_| ())
    })
}

pub fn find(repo: &Repo, query: ArticleQuery) -> Result<Vec<(Article, User, u64)>, Error> {
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
        .map(|(article, user)| {
            n_favorites(&repo, &article.slug).map(|n_fav| (article, user, n_fav as u64))
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn find_one(repo: &Repo, slug_value: &str) -> Result<domain::Article, Error> {
    use crate::db::schema::articles::dsl::{articles, slug};
    use crate::db::schema::users::dsl::users;

    //    let slug_value = slug_value.to_owned();
    let (article, user): (Article, User) = repo.run(move |conn| {
        articles
            .filter(slug.eq(slug_value))
            .inner_join(users)
            .select((articles::all_columns(), users::all_columns()))
            .first(&conn)
    })?;
    let n_fav = n_favorites(&repo, &article.slug)?;
    Ok((article, user, n_fav as u64).into())
}

pub fn feed(
    repo: &Repo,
    user_id_value: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<(Article, User, i64)>, Error> {
    use crate::db::schema::articles::dsl::{articles, created_at, user_id};
    use crate::db::schema::followers::dsl::{followed_id, follower_id, followers};
    use crate::db::schema::users::dsl::{id, users};

    let results: Vec<(Article, User)> = repo.run(move |conn| {
        followers
            .filter(follower_id.eq(user_id_value))
            .inner_join(users.on(id.eq(followed_id)))
            .inner_join(articles.on(user_id.eq(id)))
            .select((articles::all_columns(), users::all_columns()))
            .order(created_at.desc())
            .limit(limit)
            .offset(offset)
            .get_results(&conn)
    })?;
    results
        .into_iter()
        .map(|(article, user)| {
            n_favorites(&repo, &article.slug).map(|n_fav| (article, user, n_fav))
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
    let mut result: Vec<Tags> = repo.run(move |conn| query.load(&conn))?;
    // This is not actually an array: it's either a single element of empty
    let tags = match result.pop() {
        Some(tags) => HashSet::from_iter(tags.tags.into_iter()),
        None => HashSet::new(),
    };
    Ok(tags)
}
