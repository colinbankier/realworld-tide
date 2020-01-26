use crate::auth::encode_token;
use crate::domain;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub token: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl UserResponse {
    pub fn new(user: crate::db::models::User) -> Self {
        let token = encode_token(user.id);
        UserResponse {
            user: User {
                username: user.username,
                email: user.email,
                token,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

impl From<(domain::User, String)> for UserResponse {
    fn from(x: (domain::User, String)) -> Self {
        let (u, token) = x;
        Self {
            user: User {
                username: u.profile.username,
                email: u.email,
                token,
                bio: u.profile.bio,
                image: u.profile.image,
            },
        }
    }
}
