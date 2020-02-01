use crate::models::{Article, Comment, NewArticle, UpdateArticle, UpdateUser, User};
use domain;

pub fn to_article(a: Article, u: domain::User, n_fav: u64) -> domain::Article {
    let metadata = domain::ArticleMetadata {
        created_at: a.created_at,
        updated_at: a.updated_at,
    };
    let content = domain::ArticleContent {
        title: a.title,
        description: a.description,
        body: a.body,
        tag_list: a.tag_list,
    };
    domain::Article {
        content,
        slug: a.slug,
        author: u.profile,
        metadata,
        favorites_count: n_fav,
    }
}

impl From<User> for domain::User {
    fn from(u: User) -> Self {
        domain::User {
            id: u.id,
            email: u.email,
            profile: domain::Profile {
                username: u.username,
                bio: u.bio,
                image: u.image,
            },
        }
    }
}

impl From<User> for domain::Profile {
    fn from(u: User) -> Self {
        domain::Profile {
            username: u.username,
            bio: u.bio,
            image: u.image,
        }
    }
}

pub fn to_comment(c: Comment, u: User) -> domain::Comment {
    domain::Comment {
        id: c.id as u64,
        author: domain::Profile::from(u),
        body: c.body,
        created_at: c.created_at,
        updated_at: c.updated_at,
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

impl<'a> From<&'a domain::UserUpdate> for UpdateUser<'a> {
    fn from(u: &'a domain::UserUpdate) -> Self {
        Self {
            email: u.email.as_deref(),
            username: u.username.as_deref(),
            password: u.password.as_ref().map(|p| p.hash().to_owned()),
            image: u.image.as_deref(),
            bio: u.bio.as_deref(),
        }
    }
}
