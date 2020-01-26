use crate::db::models::NewFavorite;
use crate::db::schema::favorites;
use crate::domain;
use crate::domain::{FavoriteOutcome, UnfavoriteOutcome};
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;
use std::collections::HashMap;
use uuid::Uuid;

pub fn favorite(
    repo: &Repo,
    user_id: Uuid,
    article_slug: &str,
) -> Result<FavoriteOutcome, domain::DatabaseError> {
    let row = NewFavorite {
        user_id,
        article_id: article_slug.to_owned(),
    };
    let n_inserted: usize = repo.run(move |conn| {
        diesel::insert_into(favorites::table)
            .values(&row)
            // If it already exists, ignore it and don't return an error
            .on_conflict_do_nothing()
            .execute(&conn)
    })?;
    let outcome = if n_inserted == 0 {
        FavoriteOutcome::AlreadyAFavorite
    } else {
        FavoriteOutcome::NewFavorite
    };
    Ok(outcome)
}

pub fn unfavorite(
    repo: &Repo,
    user_id_value: Uuid,
    article_slug: &str,
) -> Result<UnfavoriteOutcome, domain::DatabaseError> {
    use crate::db::schema::favorites::dsl::{article_id, favorites, user_id};

    let delete = favorites.filter(article_id.eq(article_slug).and(user_id.eq(user_id_value)));
    let n_deleted: usize = repo.run(move |conn| diesel::delete(delete).execute(&conn))?;
    let outcome = if n_deleted == 0 {
        UnfavoriteOutcome::WasNotAFavorite
    } else {
        UnfavoriteOutcome::WasAFavorite
    };
    Ok(outcome)
}

/// Given a user and an article, return if the user has marked it as favorite.
pub fn is_favorite(repo: &Repo, user_id: Uuid, article_slug: &str) -> Result<bool, Error> {
    Ok(are_favorite(repo, user_id, vec![article_slug])?[article_slug])
}

/// Given a user and a list of articles, return for each of them if the user has
/// marked them as favorite.
pub fn are_favorite<'a>(
    repo: &'a Repo,
    user_id_value: Uuid,
    article_slugs: Vec<&'a str>,
) -> Result<HashMap<&'a str, bool>, Error> {
    use crate::db::schema::favorites::dsl::{article_id, favorites, user_id};

    let filter = article_id
        .eq_any(&article_slugs)
        .and(user_id.eq(user_id_value));
    let favorite_articles_ids: Vec<String> = repo.run(move |conn| {
        favorites
            .filter(filter)
            .select(article_id)
            .get_results(&conn)
    })?;

    let mut results = HashMap::new();
    for slug in article_slugs {
        results.insert(slug, favorite_articles_ids.contains(&slug.to_string()));
    }
    Ok(results)
}

/// Return the number of users who have marked a specific article as favorited.
pub fn n_favorites(repo: &Repo, article_slug: &str) -> Result<i64, Error> {
    use crate::db::schema::favorites::dsl::{article_id, favorites, user_id};
    use diesel::dsl::count;

    repo.run(move |conn| {
        favorites
            .filter(article_id.eq(article_slug))
            .select(count(user_id))
            .get_result(&conn)
    })
}
