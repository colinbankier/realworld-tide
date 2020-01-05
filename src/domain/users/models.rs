use crate::domain::repositories::{ArticleRepository, UsersRepository};
use crate::domain::{Article, ArticleView, DatabaseError};
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
