pub mod test_server;

use r2d2::CustomizeConnection;

#[derive(Debug)]
pub struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: diesel::connection::Connection,
    E: std::error::Error + Sync + Send,
{
    fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        match conn.begin_test_transaction() {
            Ok(_) => Ok(()),
            Err(_) => Ok(()), // TODO: Fix this with real error
        }
    }
}

use crate::conduit::articles;
use crate::conduit::users;
use crate::db::Repo;
use crate::models::{Article, User};
use futures::stream::{FuturesOrdered, StreamExt};

pub async fn create_users(repo: &Repo, num_users: i32) -> Vec<User> {
    let results =
        (0..num_users).map(|_| users::insert(repo.clone(), generate::new_user()) )
            .collect::<FuturesOrdered<_>>()
            .collect::<Vec<_>>().await;
    results
        .into_iter()
        .map(|r| r.expect("Failed to create user"))
        .collect()
}

pub async fn create_articles(repo: &Repo, users: Vec<User>) -> Vec<Article> {
    let results =
           users.iter().map(|user| articles::insert(repo.clone(), generate::new_article(user.id)) )
           .collect::<FuturesOrdered<_>>().collect::<Vec<_>>()
    .await;
    results
        .into_iter()
        .map(|r| r.expect("Failed to create article"))
        .collect()
}

/// Functions for generating test data
pub mod generate {
    use crate::models::{NewArticle, NewUser};
    use fake::fake;

    pub fn new_user() -> NewUser {
        NewUser {
            username: fake!(Internet.user_name).to_string(),
            email: fake!(Internet.free_email).to_string(),
            password: fake!(Lorem.word).to_string(),
        }
    }

    pub fn new_article(user_id: i32) -> NewArticle {
        NewArticle {
            title: fake!(Lorem.sentence(4, 10)).to_string(),
            slug: format!("{}{}", fake!(Lorem.word).to_string(), user_id),
            description: fake!(Lorem.paragraph(3, 10)),
            body: fake!(Lorem.paragraph(10, 5)),
            user_id: user_id,
        }
    }
}
