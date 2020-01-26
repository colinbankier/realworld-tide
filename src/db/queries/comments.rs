use crate::db::models::{Comment, NewComment, User};
use crate::db::schema::comments;
use crate::Repo;
use diesel::result::Error;
use diesel::Table;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn create_comment(repo: &Repo, comment: NewComment) -> Result<Comment, Error> {
    repo.run(move |conn| {
        diesel::insert_into(comments::table)
            .values(&comment)
            .get_result(&conn)
    })
}

pub fn get_comment(repo: &Repo, comment_id: u64) -> Result<Comment, Error> {
    use crate::db::schema::comments::dsl::comments;

    repo.run(move |conn| comments.find(comment_id as i64).get_result(&conn))
}

pub fn delete_comment(repo: &Repo, comment_id: u64) -> Result<(), Error> {
    use crate::db::schema::comments::dsl::{comments, id};

    let to_be_deleted = comments.filter(id.eq(comment_id as i64));
    repo.run(move |conn| {
        diesel::delete(to_be_deleted)
            .execute(&conn)
            // Discard the number of deleted rows
            .map(|_| ())
    })
}

pub fn get_comments(repo: &Repo, article_slug: &str) -> Result<Vec<(Comment, User)>, Error> {
    use crate::db::schema::comments::dsl::{article_id, comments};
    use crate::db::schema::users::dsl::users;

    repo.run(move |conn| {
        let q = comments
            .filter(article_id.eq(article_slug))
            .inner_join(users)
            .select((comments::all_columns(), users::all_columns()))
            .into_boxed();
        q.load(&conn)
    })
}
