use crate::domain::repositories::{ArticleRepository, UsersRepository};
use crate::domain::{
    Article, ArticleContent, ArticleUpdate, ArticleView, ChangeArticleError, Comment,
    CommentContent, DatabaseError, PublishArticleError,
};
use derive_more::Constructor;
use uuid::Uuid;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub profile: Profile,
}

impl User {
    pub fn publish(
        &self,
        draft: ArticleContent,
        repository: &impl ArticleRepository,
    ) -> Result<Article, PublishArticleError> {
        repository.publish(draft, &self)
    }

    pub fn update(
        &self,
        article: Article,
        update: ArticleUpdate,
        repository: &impl ArticleRepository,
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

    pub fn delete(
        &self,
        article: Article,
        repository: &impl ArticleRepository,
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
        repository: &impl ArticleRepository,
    ) -> Result<Comment, ChangeArticleError> {
        Ok(repository.comment_article(&self, &article, comment)?)
    }

    pub fn favorite(
        &self,
        article: Article,
        repository: &(impl ArticleRepository + UsersRepository),
    ) -> Result<ArticleView, DatabaseError> {
        let n_favorites = match repository.favorite(&article, self)? {
            FavoriteOutcome::NewFavorite => article.favorites_count + 1,
            FavoriteOutcome::AlreadyAFavorite => article.favorites_count,
        };
        let article_view = ArticleView {
            content: article.content,
            slug: article.slug,
            author: repository.get_view(self, &article.author.username)?,
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
        repository: &(impl ArticleRepository + UsersRepository),
    ) -> Result<ArticleView, DatabaseError> {
        let n_favorites = match repository.unfavorite(&article, self)? {
            UnfavoriteOutcome::WasAFavorite => article.favorites_count - 1,
            UnfavoriteOutcome::WasNotAFavorite => article.favorites_count,
        };
        let article_view = ArticleView {
            content: article.content,
            slug: article.slug,
            author: repository.get_view(self, &article.author.username)?,
            metadata: article.metadata,
            favorited: false,
            favorites_count: n_favorites,
            viewer: self.id.to_owned(),
        };
        Ok(article_view)
    }

    pub fn feed(
        &self,
        query: FeedQuery,
        repository: &impl ArticleRepository,
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

#[derive(Clone, Constructor, Debug, PartialEq)]
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
