pub mod comments;

use crate::repositories::Repository;
use uuid::Uuid;

pub struct CommandHandler<'a, R: Repository> {
    pub authenticated_user: Option<Uuid>,
    pub repository: &'a R,
}

pub trait Handle<T> {
    type Output;
    fn handle(self, command: T) -> Self::Output;
}
