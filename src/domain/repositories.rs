use crate::domain::{
    Article, ArticleContent, ArticleUpdate, ArticleView, DatabaseError, FavoriteOutcome,
    GetArticleError, GetUserError, ProfileView, PublishArticleError, UnfavoriteOutcome, User,
};
use uuid::Uuid;

pub trait ArticleRepository {
    fn publish(&self, draft: ArticleContent, author: &User)
        -> Result<Article, PublishArticleError>;
    fn get_by_slug(&self, slug: &str) -> Result<Article, GetArticleError>;
    fn get_article_view(
        &self,
        viewer: &User,
        article: Article,
    ) -> Result<ArticleView, GetArticleError>;
    fn delete_article(&self, article: &Article) -> Result<(), DatabaseError>;
    fn update_article(
        &self,
        article: Article,
        update: ArticleUpdate,
    ) -> Result<Article, DatabaseError>;
    fn favorite(&self, article: &Article, user: &User) -> Result<FavoriteOutcome, DatabaseError>;
    fn unfavorite(
        &self,
        article: &Article,
        user: &User,
    ) -> Result<UnfavoriteOutcome, DatabaseError>;
}

pub trait UsersRepository {
    fn get_by_id(&self, user_id: Uuid) -> Result<User, GetUserError>;
    fn get_view(&self, viewer: &User, username: &str) -> Result<ProfileView, GetUserError>;
}
