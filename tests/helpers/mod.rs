#![allow(dead_code)]

pub mod test_db;
pub mod test_server;

use diesel::PgConnection;
use realworld_tide::conduit::articles;
use realworld_tide::conduit::users;
use realworld_tide::db::models::{Article, User};
use realworld_tide::db::Repo;

pub fn create_users(repo: &Repo<PgConnection>, num_users: i32) -> Vec<User> {
    let results = (0..num_users)
        .map(|_| users::insert(repo, generate::new_user()))
        .collect::<Vec<_>>();
    results
        .into_iter()
        .map(|r| r.expect("Failed to create user"))
        .collect()
}

pub fn create_articles(repo: &Repo<PgConnection>, users: Vec<User>) -> Vec<Article> {
    let results = users
        .iter()
        .map(|user| articles::insert(repo, generate::new_article(user.id)))
        .collect::<Vec<_>>();
    results
        .into_iter()
        .map(|r| r.expect("Failed to create article"))
        .collect()
}

/// Functions for generating test data
pub mod generate {
    use fake::fake;
    use realworld_tide::db::models::{NewArticle, NewUser};
    use uuid::Uuid;

    pub fn new_user() -> NewUser {
        let user_id = Uuid::new_v4();
        NewUser {
            username: fake!(Internet.user_name).to_string(),
            email: fake!(Internet.free_email).to_string(),
            password: fake!(Lorem.word).to_string(),
            id: user_id,
        }
    }

    pub fn new_article(user_id: Uuid) -> NewArticle {
        NewArticle {
            title: fake!(Lorem.sentence(4, 10)).to_string(),
            slug: format!("{}{}", fake!(Lorem.word).to_string(), user_id),
            description: fake!(Lorem.paragraph(3, 10)),
            body: fake!(Lorem.paragraph(10, 5)),
            tag_list: vec![fake!(Lorem.word).to_string()],
            user_id,
        }
    }
}
