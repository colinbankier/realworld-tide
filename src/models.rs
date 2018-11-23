use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
