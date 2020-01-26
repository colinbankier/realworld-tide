//! Functions for generating test data
use fake::fake;
use realworld_tide::domain;

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

pub fn new_user() -> domain::SignUp {
    domain::SignUp {
        username: fake!(Internet.user_name).to_string(),
        email: fake!(Internet.free_email).to_string(),
        password: fake!(Lorem.word).to_string(),
    }
}
