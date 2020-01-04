//! Functions for generating test data
use fake::fake;
use realworld_tide::db::models::NewUser;
use realworld_tide::domain;
use uuid::Uuid;

pub fn article_content() -> domain::ArticleContent {
    domain::ArticleContent::new(
        fake!(Lorem.sentence(4, 10)).to_string(),
        fake!(Lorem.paragraph(3, 10)),
        fake!(Lorem.paragraph(10, 5)),
        vec![fake!(Lorem.word).to_string()],
    )
}

pub fn article_draft(author_id: Option<Uuid>) -> domain::ArticleDraft {
    domain::ArticleDraft::new(article_content(), author_id.unwrap_or(Uuid::new_v4()))
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
