use crate::conduit::{articles, users};
use crate::db::models::{Article, NewArticle};
use crate::db::Repo;
use crate::domain;
use crate::domain::{GetUserError, UsersRepository};
use diesel::PgConnection;
use uuid::Uuid;

pub struct Repository<'a>(pub &'a Repo<PgConnection>);

impl<'a> domain::ArticleRepository for Repository<'a> {
    fn publish(&self, draft: domain::ArticleDraft) -> domain::Article {
        let result: Article = articles::insert(&self.0, NewArticle::from(&draft)).unwrap();
        let metadata = domain::ArticleMetadata::new(result.created_at, result.updated_at);
        let user = self.get_by_id(draft.author_id().to_owned()).unwrap();
        domain::Article::new(
            draft.content().to_owned(),
            draft.slug(),
            user.profile().to_owned(),
            metadata,
            0,
        )
    }
}

impl<'a> domain::UsersRepository for Repository<'a> {
    fn get_by_id(&self, user_id: Uuid) -> Result<domain::User, GetUserError> {
        let u = users::find(&self.0, user_id)?;
        let profile = domain::Profile::new(u.username, u.bio, u.image);
        Ok(domain::User::new(u.id, u.email, profile))
    }
}
