use crate::repositories::Repository;
use crate::{DatabaseError, Profile, ProfileView, User};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct CommentContent(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Comment {
    pub id: u64,
    pub author: Profile,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn view(
        self,
        viewer: &User,
        repository: &impl Repository,
    ) -> Result<CommentView, DatabaseError> {
        let author_view = repository.get_profile_view(viewer, &self.author.username)?;
        let view = CommentView {
            id: self.id,
            author: author_view,
            body: self.body,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
        Ok(view)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommentView {
    pub id: u64,
    pub author: ProfileView,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
