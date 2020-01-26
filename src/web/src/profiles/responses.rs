use domain;
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

impl From<domain::Profile> for ProfileResponse {
    fn from(p: domain::Profile) -> Self {
        Self {
            profile: Profile {
                username: p.username,
                bio: p.bio,
                image: p.image,
                following: false,
            },
        }
    }
}

impl From<domain::ProfileView> for ProfileResponse {
    fn from(p: domain::ProfileView) -> Self {
        Self {
            profile: Profile {
                username: p.profile.username,
                bio: p.profile.bio,
                image: p.profile.image,
                following: p.following,
            },
        }
    }
}
