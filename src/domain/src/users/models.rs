use crate::repositories::Repository;
use crate::{
    Article, ArticleContent, ArticleUpdate, ArticleView, ChangeArticleError, Comment,
    CommentContent, CommentView, DatabaseError, DeleteCommentError, PasswordError,
    PublishArticleError,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    /// Given a clear-text password, it returns a `Password` instance
    /// containing the password's hash.
    pub fn from_clear_text(clear_text_password: String) -> Result<Password, PasswordError> {
        // TODO: Read hash cost from configuration - we are using 4 for now.
        let hash = bcrypt::hash(clear_text_password, 4)?;
        Ok(Password(hash))
    }

    /// Given an already hashed password, it returns a `Password` instance
    /// containing that very same hash.
    pub fn from_hash(hashed_password: String) -> Password {
        Password(hashed_password)
    }

    /// Returns the hashed password.
    pub fn hash(&self) -> &str {
        &self.0
    }

    /// Check that a password matches `self` when hashed.
    pub fn verify(&self, clear_text_password: &str) -> Result<bool, PasswordError> {
        Ok(bcrypt::verify(clear_text_password, &self.0)?)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub password: Password,
}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub profile: Profile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<Password>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

impl User {
    pub fn publish(
        &self,
        draft: ArticleContent,
        repository: &impl Repository,
    ) -> Result<Article, PublishArticleError> {
        repository.publish_article(draft, &self)
    }

    pub fn update_article(
        &self,
        article: Article,
        update: ArticleUpdate,
        repository: &impl Repository,
    ) -> Result<Article, ChangeArticleError> {
        if article.author.username != self.profile.username {
            return Err(ChangeArticleError::Forbidden {
                slug: article.slug,
                user_id: self.id,
            });
        }
        let updated_article = repository.update_article(article, update)?;
        Ok(updated_article)
    }

    pub fn update(
        self,
        update: UserUpdate,
        repository: &impl Repository,
    ) -> Result<Self, DatabaseError> {
        Ok(repository.update_user(self, update)?)
    }

    pub fn delete(
        &self,
        article: Article,
        repository: &impl Repository,
    ) -> Result<(), ChangeArticleError> {
        // You can only delete your own articles
        if article.author.username != self.profile.username {
            return Err(ChangeArticleError::Forbidden {
                slug: article.slug,
                user_id: self.id,
            });
        }
        Ok(repository.delete_article(&article)?)
    }

    pub fn comment(
        &self,
        article: &Article,
        comment: CommentContent,
        repository: &impl Repository,
    ) -> Result<CommentView, ChangeArticleError> {
        let posted_comment = repository.comment_article(&self, &article, comment)?;
        let view = CommentView {
            id: posted_comment.id,
            author: ProfileView {
                profile: posted_comment.author,
                // Users always self-follow
                following: true,
                viewer: self.id,
            },
            body: posted_comment.body,
            created_at: posted_comment.created_at,
            updated_at: posted_comment.updated_at,
        };
        Ok(view)
    }

    pub fn delete_comment(
        &self,
        comment: Comment,
        repository: &impl Repository,
    ) -> Result<(), DeleteCommentError> {
        // You can only delete your own comments
        if comment.author.username != self.profile.username {
            return Err(DeleteCommentError::Forbidden {
                comment_id: comment.id,
                user_id: self.id,
            });
        }

        Ok(repository.delete_comment(comment.id)?)
    }

    pub fn favorite(
        &self,
        article: Article,
        repository: &(impl Repository + Repository),
    ) -> Result<ArticleView, DatabaseError> {
        let n_favorites = match repository.favorite(&article, self)? {
            FavoriteOutcome::NewFavorite => article.favorites_count + 1,
            FavoriteOutcome::AlreadyAFavorite => article.favorites_count,
        };
        let article_view = ArticleView {
            content: article.content,
            slug: article.slug,
            author: repository.get_profile_view(self, &article.author.username)?,
            metadata: article.metadata,
            favorited: true,
            favorites_count: n_favorites,
            viewer: self.id.to_owned(),
        };
        Ok(article_view)
    }

    pub fn unfavorite(
        &self,
        article: Article,
        repository: &(impl Repository + Repository),
    ) -> Result<ArticleView, DatabaseError> {
        let n_favorites = match repository.unfavorite(&article, self)? {
            UnfavoriteOutcome::WasAFavorite => article.favorites_count - 1,
            UnfavoriteOutcome::WasNotAFavorite => article.favorites_count,
        };
        let article_view = ArticleView {
            content: article.content,
            slug: article.slug,
            author: repository.get_profile_view(self, &article.author.username)?,
            metadata: article.metadata,
            favorited: false,
            favorites_count: n_favorites,
            viewer: self.id.to_owned(),
        };
        Ok(article_view)
    }

    pub fn follow(
        &self,
        p: Profile,
        repository: &impl Repository,
    ) -> Result<ProfileView, DatabaseError> {
        repository.follow(self, &p)?;
        let view = ProfileView {
            profile: p,
            following: true,
            viewer: self.id.to_owned(),
        };
        Ok(view)
    }

    pub fn unfollow(
        &self,
        p: Profile,
        repository: &impl Repository,
    ) -> Result<ProfileView, DatabaseError> {
        repository.unfollow(self, &p)?;
        let view = ProfileView {
            profile: p,
            following: false,
            viewer: self.id.to_owned(),
        };
        Ok(view)
    }

    pub fn feed(
        &self,
        query: FeedQuery,
        repository: &impl Repository,
    ) -> Result<Vec<ArticleView>, DatabaseError> {
        Ok(repository.feed(&self, query)?)
    }
}

pub enum FavoriteOutcome {
    NewFavorite,
    AlreadyAFavorite,
}

pub enum UnfavoriteOutcome {
    WasAFavorite,
    WasNotAFavorite,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileView {
    pub profile: Profile,
    pub following: bool,
    // The user owning this view of a profile
    pub viewer: Uuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeedQuery {
    pub limit: u64,
    pub offset: u64,
}
