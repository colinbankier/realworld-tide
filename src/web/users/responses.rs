use crate::auth::encode_token;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Serialize, Debug)]
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
