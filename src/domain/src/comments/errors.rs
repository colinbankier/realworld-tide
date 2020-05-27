use crate::DatabaseError;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum DeleteCommentError {
    #[error("You have to be logged in to delete a comment.")]
    Unauthorized,
    #[error("There is no comment with {comment_id:?} as id.")]
    CommentNotFound {
        comment_id: u64,
        #[source]
        source: DatabaseError,
    },
    #[error("User {user_id:?} is not the author of the comment (id: {comment_id:?}).")]
    Forbidden { user_id: Uuid, comment_id: u64 },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}
