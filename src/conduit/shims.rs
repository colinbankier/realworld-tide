use crate::db::models::{Article, User};
use crate::domain;

impl From<(Article, User, u64)> for domain::Article {
    fn from(x: (Article, User, u64)) -> Self {
        let (a, u, n_fav) = x;
        let metadata = domain::ArticleMetadata::new(a.created_at, a.updated_at);
        let content = domain::ArticleContent::new(a.title, a.description, a.body, a.tag_list);
        let user: domain::User = u.into();
        domain::Article::new(content, a.slug, user.profile.to_owned(), metadata, n_fav)
    }
}

impl From<User> for domain::User {
    fn from(u: User) -> Self {
        let profile = domain::Profile::new(u.username, u.bio, u.image);
        domain::User::new(u.id, u.email, profile)
    }
}
