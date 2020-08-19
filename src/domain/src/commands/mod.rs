use crate::repositories::Repository;
use crate::{ArticleNotFoundError, ChangeArticleError, CommentContent, CommentView, DatabaseError};
use crate::{GetArticleError, GetUserError};
use uuid::Uuid;

pub struct CommandHandler<'a, R: Repository> {
    pub authenticated_user: Option<Uuid>,
    pub repository: &'a R,
}

pub trait Handle<T> {
    type Output;
    fn handle(self, command: T) -> Self::Output;
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

impl<'a, R: Repository> Handle<CreateComment> for CommandHandler<'a, R> {
    type Output = Result<CommentView, CreateCommentError>;

    fn handle(self, command: CreateComment) -> Self::Output {
        let author_id = self
            .authenticated_user
            .ok_or(CreateCommentError::Unauthorized)?;
        let author = self.repository.get_user_by_id(author_id)?;
        let article = self.repository.get_article_by_slug(&command.article_slug)?;
        let comment = author.comment(
            &article,
            CommentContent(command.comment_body),
            self.repository,
        )?;
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
