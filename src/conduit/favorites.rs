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
