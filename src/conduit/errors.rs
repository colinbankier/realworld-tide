use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub enum InternalError {
    RepoError(DieselError),
    HashError(BcryptError),
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RepoError(e) => write!(f, "Diesel Error: {}", e),
            Self::HashError(e) => write!(f, "Bcrypt Error: {}", e),
        }
    }
}

impl From<DieselError> for InternalError {
    fn from(err: DieselError) -> Self {
        Self::RepoError(err)
    }
}

impl From<BcryptError> for InternalError {
    fn from(err: BcryptError) -> Self {
        Self::HashError(err)
    }
}

#[derive(Debug)]
pub enum FindError {
    NotFound,
    Internal(InternalError),
}

impl fmt::Display for FindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Error: Not Found"),
            Self::Internal(e) => write!(f, "{}", e),
        }
    }
}

impl From<DieselError> for FindError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => Self::NotFound,
            _ => Self::Internal(InternalError::RepoError(err)),
        }
    }
}

impl From<BcryptError> for FindError {
    fn from(err: BcryptError) -> Self {
        Self::Internal(InternalError::HashError(err))
    }
}
