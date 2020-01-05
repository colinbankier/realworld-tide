use crate::db::models::{Article, NewArticle, UpdateArticle, User};
use crate::domain;

impl From<(Article, User, u64)> for domain::Article {
    fn from(x: (Article, User, u64)) -> Self {
        let (a, u, n_fav) = x;
        let metadata = domain::ArticleMetadata::new(a.created_at, a.updated_at);
        let content = domain::ArticleContent::new(a.title, a.description, a.body, a.tag_list);
        let user: domain::User = u.into();
        domain::Article::new(content, a.slug, user.profile, metadata, n_fav)
    }
}

impl From<User> for domain::User {
    fn from(u: User) -> Self {
        let profile = domain::Profile::new(u.username, u.bio, u.image);
        domain::User::new(u.id, u.email, profile)
    }
}

impl<'a> From<(&'a domain::ArticleContent, &'a domain::User)> for NewArticle<'a> {
    fn from(x: (&'a domain::ArticleContent, &'a domain::User)) -> Self {
        let (draft, author) = x;
        Self {
            title: &draft.title,
            slug: draft.slug(),
            description: &draft.description,
            body: &draft.body,
            tag_list: draft.tag_list.to_owned(),
            user_id: author.id.to_owned(),
        }
    }
}

impl<'a> From<&'a domain::ArticleUpdate> for UpdateArticle<'a> {
    fn from(update: &'a domain::ArticleUpdate) -> Self {
        Self {
            title: update.title.as_deref(),
            description: update.description.as_deref(),
            body: update.body.as_deref(),
        }
    }
}
