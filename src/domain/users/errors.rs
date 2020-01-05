use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum GetUserError {
    #[error("There is no user with id {user_id:?}.")]
    NotFound {
        user_id: Uuid,
        source: diesel::result::Error,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] diesel::result::Error),
}
