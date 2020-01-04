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
    domain::ArticleContent::new(
        fake!(Lorem.sentence(4, 10)).to_string(),
        fake!(Lorem.paragraph(3, 10)),
        fake!(Lorem.paragraph(10, 5)),
        vec![fake!(Lorem.word).to_string()],
    )
}

pub fn article_draft(author_id: With<Uuid>) -> domain::ArticleDraft {
    let author_id = match author_id {
        With::Value(id) => id,
        With::Random => Uuid::new_v4(),
    };
    domain::ArticleDraft::new(article_content(), author_id)
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
