use crate::conduit::{articles, users};
use crate::db::models::{Article, NewArticle};
use crate::db::Repo;
use crate::domain;
use crate::domain::{GetUserError, UsersRepository};
use diesel::PgConnection;
use uuid::Uuid;

pub struct Repository<'a>(pub &'a Repo<PgConnection>);

impl<'a> domain::ArticleRepository for Repository<'a> {
    fn publish(
        &self,
        draft: domain::ArticleDraft,
    ) -> Result<domain::Article, domain::PublishArticleError> {
        let user = self.get_by_id(draft.author_id().to_owned())?;

        let result: Article = articles::insert(&self.0, NewArticle::from(&draft))?;

        let metadata = domain::ArticleMetadata::new(result.created_at, result.updated_at);
        let article = domain::Article::new(
            draft.content().to_owned(),
            draft.slug(),
            user.profile().to_owned(),
            metadata,
            0,
        );
        Ok(article)
    }
}

impl<'a> domain::UsersRepository for Repository<'a> {
    fn get_by_id(&self, user_id: Uuid) -> Result<domain::User, GetUserError> {
        let u = users::find(&self.0, user_id)?;
        let profile = domain::Profile::new(u.username, u.bio, u.image);
        Ok(domain::User::new(u.id, u.email, profile))
    }
}
