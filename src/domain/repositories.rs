use crate::domain::{
    Article, ArticleContent, ArticleQuery, ArticleUpdate, ArticleView, Comment, CommentContent,
    DatabaseError, DeleteCommentError, FavoriteOutcome, FeedQuery, GetArticleError, GetUserError,
    Profile, ProfileView, PublishArticleError, UnfavoriteOutcome, User,
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
    fn get_articles_views(
        &self,
        viewer: &User,
        articles: Vec<Article>,
    ) -> Result<Vec<ArticleView>, DatabaseError>;
    fn find_articles(&self, query: ArticleQuery) -> Result<Vec<Article>, DatabaseError>;
    fn feed(&self, user: &User, query: FeedQuery) -> Result<Vec<ArticleView>, DatabaseError>;
    fn delete_article(&self, article: &Article) -> Result<(), DatabaseError>;
    fn comment_article(
        &self,
        user: &User,
        article: &Article,
        comment: CommentContent,
    ) -> Result<Comment, DatabaseError>;
    fn get_comment(&self, comment_id: u64) -> Result<Comment, DeleteCommentError>;
    fn get_comments(&self, article: &Article) -> Result<Vec<Comment>, DatabaseError>;
    fn delete_comment(&self, comment_id: u64) -> Result<(), DeleteCommentError>;
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
    fn get_profile(&self, username: &str) -> Result<Profile, GetUserError>;
    fn get_view(&self, viewer: &User, username: &str) -> Result<ProfileView, GetUserError>;
    fn follow(&self, follower: &User, to_be_followed: &Profile) -> Result<(), DatabaseError>;
    fn unfollow(&self, follower: &User, to_be_unfollowed: &Profile) -> Result<(), DatabaseError>;
}
