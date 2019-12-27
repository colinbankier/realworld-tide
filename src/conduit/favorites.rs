use crate::db::models::NewFavorite;
use crate::db::schema::favorites;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;
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
