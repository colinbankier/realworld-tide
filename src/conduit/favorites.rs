use crate::db::models::NewFavorite;
use crate::db::schema::favorites;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;
use std::collections::HashMap;
use uuid::Uuid;

pub fn favorite(repo: &Repo, user_id: Uuid, article_id: i32) -> Result<(), Error> {
    let row = NewFavorite {
        user_id,
        article_id,
    };
    repo.run(move |conn| {
        diesel::insert_into(favorites::table)
            .values(&row)
            // If it already exists, ignore it and don't return an error
            .on_conflict_do_nothing()
            .execute(&conn)
            // Discard the number of inserted rows
            .map(|_| ())
    })
}

/// Given a user and an article, return if the user has marked it as favorite.
pub fn is_favorite(repo: &Repo, user_id: Uuid, article_id: i32) -> Result<bool, Error> {
    Ok(are_favorite(repo, user_id, vec![article_id])?[&article_id])
}

/// Given a user and a list of articles, return for each of them if the user has
/// marked them as favorite.
pub fn are_favorite(
    repo: &Repo,
    user_id_value: Uuid,
    article_ids: Vec<i32>,
) -> Result<HashMap<i32, bool>, Error> {
    use crate::db::schema::favorites::dsl::{article_id, favorites, user_id};

    let filter = article_id
        .eq_any(article_ids.clone())
        .and(user_id.eq(user_id_value));
    let favorite_articles_ids: Vec<i32> = repo.run(move |conn| {
        favorites
            .filter(filter)
            .select(article_id)
            .get_results(&conn)
    })?;

    let mut results = HashMap::with_capacity(article_ids.len());
    for id in &article_ids {
        results.insert(id.to_owned(), favorite_articles_ids.contains(id));
    }
    Ok(results)
}

/// Return the number of users who have marked a specific article as favorited.
pub fn n_favorites(repo: &Repo, article_id_value: i32) -> Result<i64, Error> {
    use crate::db::schema::favorites::dsl::{article_id, favorites, user_id};
    use diesel::dsl::count;

    repo.run(move |conn| {
        favorites
            .filter(article_id.eq(article_id_value))
            .select(count(user_id))
            .get_result(&conn)
    })
}
