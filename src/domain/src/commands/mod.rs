use crate::repositories::Repository;
use crate::{ChangeArticleError, CommentContent, CommentView, DatabaseError};
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
    #[error("There is no article with {slug:?} as slug.")]
    ArticleNotFound {
        slug: String,
        #[source]
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}

impl Command for CreateComment {
    type Output = Result<CommentView, CreateCommentError>;

    fn execute<R: Repository>(self, c: CommandContext<R>) -> Self::Output {
        match c.authenticated_user {
            Some(author_id) => {
                let author = c
                    .repository
                    .get_user_by_id(author_id)
                    .map_err(|e| match e {
                        GetUserError::NotFound { user_id, source } => {
                            CreateCommentError::AuthorNotFound {
                                author_id: user_id,
                                source,
                            }
                        }
                        GetUserError::DatabaseError(e) => e.into(),
                    })?;
                let article = c
                    .repository
                    .get_article_by_slug(&self.article_slug)
                    .map_err(|e| match e {
                        GetArticleError::ArticleNotFound { slug, source } => {
                            CreateCommentError::ArticleNotFound { slug, source }
                        }
                        GetArticleError::DatabaseError(e) => e.into(),
                    })?;
                let comment = author
                    .comment(&article, CommentContent(self.comment_body), c.repository)
                    .map_err(|e| match e {
                        ChangeArticleError::ArticleNotFound { slug, source } => {
                            CreateCommentError::ArticleNotFound { slug, source }
                        }
                        ChangeArticleError::Forbidden { .. } => panic!("Impossible."),
                        ChangeArticleError::DatabaseError(e) => e.into(),
                    })?;
                Ok(comment)
            }
            None => Err(CreateCommentError::Unauthorized),
        }
    }
}
