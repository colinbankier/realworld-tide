#![allow(dead_code)]

pub mod generate;
pub mod test_db;
pub mod test_server;

use diesel::PgConnection;
use realworld_tide::conduit::articles;
use realworld_tide::conduit::users;
use realworld_tide::db::models::{Article, NewArticle, User};
use realworld_tide::db::Repo;

pub fn create_users(repo: &Repo<PgConnection>, num_users: i32) -> Vec<User> {
    (0..num_users).map(|_| create_user(repo)).collect()
}

pub fn create_user(repo: &Repo<PgConnection>) -> User {
    users::insert(repo, generate::new_user()).expect("Failed to create user")
}

pub fn create_articles(repo: &Repo<PgConnection>, users: Vec<User>) -> Vec<Article> {
    users
        .iter()
        .map(|user| create_article(repo, &user))
        .collect::<Vec<_>>()
}

pub fn create_article(repo: &Repo<PgConnection>, user: &User) -> Article {
    let draft = generate::article_draft(generate::With::Value(user.id));
    articles::insert(repo, NewArticle::from(&draft)).expect("Failed to create articles")
}
