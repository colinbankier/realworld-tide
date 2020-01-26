//! Functions for generating test data
use fake::fake;
use realworld_tide::db::models::NewUser;
use realworld_tide::domain;
use uuid::Uuid;

pub enum With<T> {
    Value(T),
    Random,
}

pub fn article_content() -> domain::ArticleContent {
    domain::ArticleContent {
        title: fake!(Lorem.sentence(4, 10)).to_string(),
        description: fake!(Lorem.paragraph(3, 10)),
        body: fake!(Lorem.paragraph(10, 5)),
        tag_list: vec![fake!(Lorem.word).to_string()],
    }
}

pub fn new_user() -> NewUser {
    let user_id = Uuid::new_v4();
    NewUser {
        username: fake!(Internet.user_name).to_string(),
        email: fake!(Internet.free_email).to_string(),
        password: fake!(Lorem.word).to_string(),
        id: user_id,
    }
}
