#![allow(dead_code)]

pub mod generate;
pub mod test_db;
pub mod test_server;

use crate::helpers::generate::With;
use diesel::PgConnection;
use realworld_tide::conduit::articles;
use realworld_tide::conduit::articles_repository::Repository;
use realworld_tide::conduit::users;
use realworld_tide::db::models::{Article, NewArticle, NewUser, User};
use realworld_tide::db::Repo;
use realworld_tide::domain;
use realworld_tide::domain::repositories::Repository as RepositoryTrait;
use uuid::Uuid;

pub fn create_users(repo: &Repo<PgConnection>, num_users: i32) -> Vec<User> {
    (0..num_users).map(|_| create_user(repo)).collect()
}

pub fn create_users2(repo: &Repo<PgConnection>, num_users: i32) -> Vec<domain::User> {
    (0..num_users).map(|_| create_user2(repo)).collect()
}

pub fn create_user(repo: &Repo<PgConnection>) -> User {
    let sign_up = generate::new_user();
    let new_user = NewUser {
        username: &sign_up.username,
        email: &sign_up.email,
        password: &sign_up.password,
        id: Uuid::new_v4(),
    };
    users::insert(&repo, new_user).expect("Failed to create user")
}

pub fn create_user2(repo: &Repo<PgConnection>) -> domain::User {
    let repository = Repository(&repo);
    repository
        .sign_up(generate::new_user())
        .expect("Failed to create user")
        .into()
}

pub fn create_articles(repo: &Repo<PgConnection>, users: Vec<User>) -> Vec<Article> {
    users
        .iter()
        .map(|user| create_article(repo, &user))
        .collect::<Vec<_>>()
}

pub fn create_article(repo: &Repo<PgConnection>, user: &User) -> Article {
    let draft = generate::article_content();
    let author: User = user.to_owned();
    articles::insert(repo, NewArticle::from((&draft, &author.into())))
        .expect("Failed to create articles")
}

pub fn create_article2(repo: &Repo<PgConnection>, author: With<&domain::User>) -> domain::Article {
    let author = match author {
        With::Random => create_user2(&repo),
        With::Value(user) => user.to_owned(),
    };
    let draft = generate::article_content();
    let repository = Repository(&repo);
    author.publish(draft, &repository).unwrap()
}
