use crate::conduit::{articles, comments, favorites, followers, users};
use crate::db::models::{Article, NewArticle, NewComment, NewUser, UpdateUser};
use crate::db::Repo;
use crate::domain;
use crate::domain::{DatabaseError, DeleteCommentError, GetUserError};
use diesel::result::Error;
use diesel::PgConnection;
use std::collections::HashSet;
use uuid::Uuid;

pub struct Repository(pub Repo<PgConnection>);

impl domain::repositories::Repository for Repository {
    fn publish_article(
        &self,
        draft: domain::ArticleContent,
        author: &domain::User,
    ) -> Result<domain::Article, domain::PublishArticleError> {
        let result: Article = articles::insert(&self.0, NewArticle::from((&draft, author)))?;
        let article = (result, author.to_owned(), 0).into();
        Ok(article)
    }

    fn get_article_by_slug(&self, slug: &str) -> Result<domain::Article, domain::GetArticleError> {
        Ok(articles::find_one(&self.0, &slug)?)
    }

    fn get_article_view(
        &self,
        viewer: &domain::User,
        article: domain::Article,
    ) -> Result<domain::ArticleView, domain::GetArticleError> {
        let author_view = self
            .get_profile_view(viewer, &article.author.username)
            .unwrap();
        let is_favorite = favorites::is_favorite(&self.0, viewer.id, &article.slug)?;
        let article_view = domain::ArticleView {
            content: article.content,
            slug: article.slug,
            author: author_view,
            metadata: article.metadata,
            favorited: is_favorite,
            favorites_count: article.favorites_count,
            viewer: viewer.id,
        };
        Ok(article_view)
    }

    fn get_articles_views(
        &self,
        viewer: &domain::User,
        articles: Vec<domain::Article>,
    ) -> Result<Vec<domain::ArticleView>, DatabaseError> {
        let slugs: Vec<String> = articles.iter().map(|a| a.slug.to_owned()).collect();
        let slugs: Vec<&str> = slugs.iter().map(|slug| slug.as_str()).collect();

        let favs = favorites::are_favorite(&self.0, viewer.id, slugs)?;
        articles
            .into_iter()
            .map(|a| {
                let favorited = favs[a.slug.as_str()];
                let author_view = self.get_profile_view(viewer, &a.author.username)?;
                let article_view = domain::ArticleView {
                    content: a.content,
                    slug: a.slug,
                    author: author_view,
                    metadata: a.metadata,
                    favorited,
                    favorites_count: a.favorites_count,
                    viewer: viewer.id,
                };
                Ok(article_view)
            })
            .collect()
    }

    fn find_articles(
        &self,
        query: domain::ArticleQuery,
    ) -> Result<Vec<domain::Article>, DatabaseError> {
        let result: Vec<domain::Article> = articles::find(&self.0, query)?
            .into_iter()
            .map(|a| a.into())
            .collect();
        Ok(result)
    }

    fn feed(
        &self,
        user: &domain::User,
        query: domain::FeedQuery,
    ) -> Result<Vec<domain::ArticleView>, DatabaseError> {
        let articles: Vec<domain::Article> =
            articles::feed(&self.0, user.id, query.limit, query.offset)?
                .into_iter()
                .map(|a| a.into())
                .collect();
        Ok(self.get_articles_views(user, articles)?)
    }

    fn delete_article(&self, article: &domain::Article) -> Result<(), DatabaseError> {
        Ok(articles::delete(&self.0, &article.slug)?)
    }

    fn comment_article(
        &self,
        user: &domain::User,
        article: &domain::Article,
        comment: domain::CommentContent,
    ) -> Result<domain::Comment, DatabaseError> {
        let new_comment = NewComment {
            body: &comment.0,
            article_id: &article.slug,
            author_id: user.id,
        };
        let raw_comment = comments::create_comment(&self.0, new_comment)?;
        let comment = domain::Comment {
            id: raw_comment.id as u64,
            author: user.profile.clone(),
            body: raw_comment.body,
            created_at: raw_comment.created_at,
            updated_at: raw_comment.updated_at,
        };
        Ok(comment)
    }

    fn get_comment(&self, comment_id: u64) -> Result<domain::Comment, DeleteCommentError> {
        let comment = comments::get_comment(&self.0, comment_id).map_err(|e| match e {
            Error::NotFound => DeleteCommentError::CommentNotFound {
                comment_id,
                source: e,
            },
            e => DeleteCommentError::DatabaseError(e),
        })?;
        let author = users::find(&self.0, comment.author_id)?;
        Ok(domain::Comment::from((comment, author)))
    }

    fn get_comments(
        &self,
        article: &domain::Article,
    ) -> Result<Vec<domain::Comment>, DatabaseError> {
        let comments: Vec<_> = comments::get_comments(&self.0, &article.slug)?
            .into_iter()
            .map(|(c, u)| domain::Comment::from((c, u)))
            .collect();
        Ok(comments)
    }

    fn delete_comment(&self, comment_id: u64) -> Result<(), DeleteCommentError> {
        Ok(comments::delete_comment(&self.0, comment_id)?)
    }

    fn update_article(
        &self,
        article: domain::Article,
        update: domain::ArticleUpdate,
    ) -> Result<domain::Article, DatabaseError> {
        articles::update(&self.0, (&update).into(), &article.slug)?;
        Ok(self.get_article_by_slug(&article.slug)?)
    }

    fn favorite(
        &self,
        article: &domain::Article,
        user: &domain::User,
    ) -> Result<domain::FavoriteOutcome, domain::DatabaseError> {
        favorites::favorite(&self.0, user.id, &article.slug)
    }

    fn unfavorite(
        &self,
        article: &domain::Article,
        user: &domain::User,
    ) -> Result<domain::UnfavoriteOutcome, domain::DatabaseError> {
        favorites::unfavorite(&self.0, user.id, &article.slug)
    }

    fn sign_up(&self, sign_up: domain::SignUp) -> Result<domain::User, domain::SignUpError> {
        let new_user = NewUser {
            username: &sign_up.username,
            email: &sign_up.email,
            password: &sign_up.password,
            id: Uuid::new_v4(),
        };
        Ok(users::insert(&self.0, new_user)?.into())
    }

    fn update_user(
        &self,
        user: domain::User,
        update: domain::UserUpdate,
    ) -> Result<domain::User, DatabaseError> {
        let update = UpdateUser::from(&update);
        let updated = users::update(&self.0, user.id, update)?;
        Ok(domain::User::from(updated))
    }

    fn get_user_by_id(&self, user_id: Uuid) -> Result<domain::User, GetUserError> {
        let result = users::find(&self.0, user_id);
        let user = result.map_err(|e| match e {
            e @ Error::NotFound => domain::GetUserError::NotFound { user_id, source: e },
            e => domain::GetUserError::DatabaseError(e),
        })?;
        Ok(domain::User::from(user))
    }

    fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<domain::User, domain::LoginError> {
        let result = users::find_by_email_password(&self.0, email, password);
        let user = result.map_err(|e| match e {
            Error::NotFound => domain::LoginError::NotFound,
            e => domain::LoginError::DatabaseError(e),
        })?;
        Ok(domain::User::from(user))
    }

    fn get_profile(&self, username: &str) -> Result<domain::Profile, GetUserError> {
        let user = users::find_by_username(&self.0, username)?;
        Ok(domain::Profile::from(user))
    }

    fn get_profile_view(
        &self,
        viewer: &domain::User,
        username: &str,
    ) -> Result<domain::ProfileView, GetUserError> {
        let viewed_user = users::find_by_username(&self.0, username)?;
        let following = followers::is_following(&self.0, viewer.id, viewed_user.id)?;
        let view = domain::ProfileView {
            profile: domain::Profile::from(viewed_user),
            following,
            viewer: viewer.id,
        };
        Ok(view)
    }

    fn follow(
        &self,
        follower: &domain::User,
        to_be_followed: &domain::Profile,
    ) -> Result<(), DatabaseError> {
        let followed_user = users::find_by_username(&self.0, &to_be_followed.username)?;
        Ok(followers::follow(&self.0, follower.id, followed_user.id)?)
    }

    fn unfollow(
        &self,
        follower: &domain::User,
        to_be_unfollowed: &domain::Profile,
    ) -> Result<(), DatabaseError> {
        let unfollowed_user = users::find_by_username(&self.0, &to_be_unfollowed.username)?;
        Ok(followers::unfollow(
            &self.0,
            follower.id,
            unfollowed_user.id,
        )?)
    }

    fn get_tags(&self) -> Result<HashSet<String>, DatabaseError> {
        Ok(articles::tags(&self.0)?)
    }
}
