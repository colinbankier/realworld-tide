use crate::domain::GetUserError;

#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DatabaseError {
    #[from]
    source: diesel::result::Error,
}

impl From<GetUserError> for DatabaseError {
    fn from(e: GetUserError) -> Self {
        match e {
            GetUserError::NotFound { source, .. } => DatabaseError { source },
            GetUserError::DatabaseError(e) => DatabaseError { source: e },
        }
    }
}
