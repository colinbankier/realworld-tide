use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProfileResponse {
    pub profile: Profile,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl ProfileResponse {
    pub fn new(user: crate::db::models::User, following: bool) -> Self {
        Self {
            profile: Profile {
                username: user.username,
                bio: user.bio,
                image: user.image,
                following,
            },
        }
    }
}
