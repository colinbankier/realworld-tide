#[cfg(test)]
use dotenv;
use r2d2::CustomizeConnection;

pub fn init_env() {
    dotenv::from_filename(".env.test").ok();
}

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

/// Functions for generating test data
pub mod generate {
    use crate::models::{NewUser, NewArticle};
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
            slug: fake!(Lorem.word).to_string(),
            description: fake!(Lorem.paragraph(3, 10)),
            body: fake!(Lorem.paragraph(10, 5)),
            user_id: user_id,
        }
    }
}
