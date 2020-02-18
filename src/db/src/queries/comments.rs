use crate::models::{Comment, NewComment, User};
use crate::schema::comments;
use crate::Repo;
use diesel::result::Error;
use diesel::Table;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn create_comment(repo: &Repo, comment: NewComment) -> Result<Comment, Error> {
    diesel::insert_into(comments::table)
        .values(&comment)
        .get_result(&repo.conn())
}

pub fn get_comment(repo: &Repo, comment_id: u64) -> Result<Comment, Error> {
    use crate::schema::comments::dsl::comments;

    comments.find(comment_id as i64).get_result(&repo.conn())
}

pub fn delete_comment(repo: &Repo, comment_id: u64) -> Result<(), Error> {
    use crate::schema::comments::dsl::{comments, id};

    let to_be_deleted = comments.filter(id.eq(comment_id as i64));
    diesel::delete(to_be_deleted)
        .execute(&repo.conn())
        // Discard the number of deleted rows
        .map(|_| ())
}

pub fn get_comments(repo: &Repo, article_slug: &str) -> Result<Vec<(Comment, User)>, Error> {
    use crate::schema::comments::dsl::{article_id, comments};
    use crate::schema::users::dsl::users;

    comments
        .filter(article_id.eq(article_slug))
        .inner_join(users)
        .select((comments::all_columns(), users::all_columns()))
        .load(&repo.conn())
}
