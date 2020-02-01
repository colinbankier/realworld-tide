#![allow(dead_code)]

pub mod generate;
pub mod test_db;

use realworld_db::models::{Article, NewArticle, NewUser, User};
use realworld_db::queries::{articles, users};
use realworld_db::Repo;
use uuid::Uuid;

pub fn create_users(repo: &Repo, num_users: i32) -> Vec<(User, String)> {
    (0..num_users).map(|_| create_user(repo)).collect()
}

pub fn create_user(repo: &Repo) -> (User, String) {
    let (sign_up, clear_text_password) = generate::new_user();
    let new_user = NewUser {
        username: &sign_up.username,
        email: &sign_up.email,
        password: &sign_up.password.hash(),
        id: Uuid::new_v4(),
    };
    (
        users::insert(&repo, new_user).expect("Failed to create user"),
        clear_text_password,
    )
}

pub fn create_articles(repo: &Repo, users: Vec<User>) -> Vec<Article> {
    users
        .iter()
        .map(|user| create_article(repo, &user))
        .collect::<Vec<_>>()
}

pub fn create_article(repo: &Repo, user: &User) -> Article {
    let draft = generate::article_content();
    let author: User = user.to_owned();
    articles::insert(repo, NewArticle::from((&draft, &author.into())))
        .expect("Failed to create articles")
}
