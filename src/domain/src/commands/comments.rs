use crate::commands::{CommandHandler, Handle};
use crate::comments::{CreateCommentError, DeleteCommentError};
use crate::repositories::Repository;
use crate::{ChangeArticleError, CommentContent, CommentView};
use crate::{GetArticleError, GetUserError};

pub struct CreateComment {
    pub article_slug: String,
    pub comment_body: String,
}

pub struct DeleteComment {
    pub comment_id: u64,
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

impl<'a, R: Repository> Handle<DeleteComment> for CommandHandler<'a, R> {
    type Output = Result<(), DeleteCommentError>;

    fn handle(self, command: DeleteComment) -> Self::Output {
        let author_id = self
            .authenticated_user
            .ok_or(DeleteCommentError::Unauthorized)?;
        let author = self.repository.get_user_by_id(author_id)?;
        let comment = self.repository.get_comment(command.comment_id)?;
        author.delete_comment(comment, self.repository)?;
        Ok(())
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

impl From<GetUserError> for DeleteCommentError {
    fn from(e: GetUserError) -> Self {
        match e {
            GetUserError::NotFound {
                user_id: _,
                source: _,
            } => DeleteCommentError::Unauthorized,
            GetUserError::DatabaseError(e) => e.into(),
        }
    }
}
