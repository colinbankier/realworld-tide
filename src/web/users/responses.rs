use crate::models::User;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: User,
}
