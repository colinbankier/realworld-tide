#![allow(dead_code)]

pub mod generate;
pub mod test_db;
pub mod test_server;

use crate::helpers::generate::With;
use db::models::{Article, NewArticle, NewUser, User};
use db::queries::{articles, users};
use db::{Repo, Repository};
use domain::repositories::Repository as RepositoryTrait;
use uuid::Uuid;

pub fn create_users(repo: &Repo, num_users: i32) -> Vec<User> {
    (0..num_users).map(|_| create_user(repo)).collect()
}

pub fn create_users2(repo: &Repository, num_users: i32) -> Vec<domain::User> {
    (0..num_users).map(|_| create_user2(repo)).collect()
}

pub fn create_user(repo: &Repo) -> User {
    let sign_up = generate::new_user();
    let new_user = NewUser {
        username: &sign_up.username,
        email: &sign_up.email,
        password: &sign_up.password,
        id: Uuid::new_v4(),
    };
    users::insert(&repo, new_user).expect("Failed to create user")
}

pub fn create_user2(repo: &Repository) -> domain::User {
    repo.sign_up(generate::new_user())
        .expect("Failed to create user")
        .into()
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

pub fn create_article2(repo: &Repository, author: With<&domain::User>) -> domain::Article {
    let author = match author {
        With::Random => create_user2(repo),
        With::Value(user) => user.to_owned(),
    };
    let draft = generate::article_content();
    author.publish(draft, repo).unwrap()
}
