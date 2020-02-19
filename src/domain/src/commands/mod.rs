use crate::repositories::Repository;
use crate::{ArticleNotFoundError, ChangeArticleError, CommentContent, CommentView, DatabaseError};
use crate::{GetArticleError, GetUserError};
use uuid::Uuid;

pub struct CommandContext<'a, R: Repository> {
    pub authenticated_user: Option<Uuid>,
    pub repository: &'a R,
}

pub trait Command {
    type Output;
    fn execute<R: Repository>(self, c: CommandContext<R>) -> Self::Output;
}

pub struct CreateComment {
    pub article_slug: String,
    pub comment_body: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateCommentError {
    #[error("You have to be logged in to post a comment.")]
    Unauthorized,
    #[error("There is no user with {author_id:?} as id.")]
    AuthorNotFound {
        author_id: Uuid,
        #[source]
        source: DatabaseError,
    },
    #[error("{0}")]
    ArticleNotFound(#[from] ArticleNotFoundError),
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}

impl Command for CreateComment {
    type Output = Result<CommentView, CreateCommentError>;

    fn execute<R: Repository>(self, c: CommandContext<R>) -> Self::Output {
        let author_id = c
            .authenticated_user
            .ok_or(CreateCommentError::Unauthorized)?;
        let author = c.repository.get_user_by_id(author_id)?;
        let article = c.repository.get_article_by_slug(&self.article_slug)?;
        let comment = author.comment(&article, CommentContent(self.comment_body), c.repository)?;
        Ok(comment)
    }
}

impl From<ChangeArticleError> for CreateCommentError {
    fn from(e: ChangeArticleError) -> Self {
        match e {
            ChangeArticleError::ArticleNotFound(e) => e.into(),
            ChangeArticleError::Forbidden { .. } => panic!("Impossible."),
            ChangeArticleError::DatabaseError(e) => e.into(),
        }
    }
}

impl From<GetUserError> for CreateCommentError {
    fn from(e: GetUserError) -> Self {
        match e {
            GetUserError::NotFound { user_id, source } => CreateCommentError::AuthorNotFound {
                author_id: user_id,
                source,
            },
            GetUserError::DatabaseError(e) => e.into(),
        }
    }
}

impl From<GetArticleError> for CreateCommentError {
    fn from(e: GetArticleError) -> Self {
        match e {
            GetArticleError::ArticleNotFound(e) => e.into(),
            GetArticleError::DatabaseError(e) => e.into(),
        }
    }
}
