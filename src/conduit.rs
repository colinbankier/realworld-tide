use crate::db::Repo;
use crate::models::User;
use crate::models::*;
use crate::schema::articles;
use crate::schema::users;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_user(repo: Repo, user: NewUser) -> Result<User, Error> {
    await! { repo.run(move |conn| {
        // TODO: store password not in plain text, later
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
    })}
}

pub async fn find_user(repo: Repo, user_id: i32) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    await! { repo.run(move |conn| users.find(user_id).first(&conn)) }
}

pub async fn authenticate_user(repo: Repo, email: String, password: String) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    await! { repo.run(|conn| {
    users
        .filter(email.eq(email))
        .filter(password.eq(password))
        .first::<User>(&conn)
    }) }
}

pub async fn update_user(repo: Repo, user_id: i32, details: UpdateUser) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    await! { repo.run(move |conn| {
        diesel::update(users.find(user_id))
            .set(&details)
            .get_result(&conn)
    })}
}

pub async fn create_article(repo: Repo, article: NewArticle) -> Result<Article, Error> {
    await! { repo.run(move |conn| {
        diesel::insert_into(articles::table)
            .values(&article)
            .get_result(&conn)
    })}
}

pub async fn list_articles(repo: Repo) -> Result<Vec<Article>, Error> {
    use crate::schema::articles::dsl::*;
    await! { repo.run(move |conn| articles.load(&conn)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::users;
    use crate::schema::users::dsl::*;
    use crate::test_helpers::generate;
    use crate::test_helpers::init_env;
    use diesel::prelude::*;
    use fake::fake;
    use futures::future;
    use futures::stream::{FuturesOrdered, StreamExt};
    use tokio_async_await;
    use tokio_async_await_test::async_test;

    #[async_test]
    async fn test_create_user() {
        init_env();
        let repo = Repo::new();
        // Create a new user
        let new_user = generate::new_user();
        let user = await! { create_user(repo.clone(), new_user) }.expect("Create user failed.");

        // Check the user is in the database.
        let results = await! {
           find_user(repo.clone(), user.id)
        };
        assert!(results.is_ok());
    }

    #[async_test]
    async fn test_authenticate_user() {
        init_env();
        let repo = Repo::new();
        // Create a new user
        let new_user = generate::new_user();
        let user = await! { create_user(repo.clone(), new_user) }.expect("Create user failed.");

        // Check the user is in the database.
        let results = await! {
           authenticate_user(repo.clone(), user.email, user.password)
        };
        assert!(results.is_ok());
    }

    #[async_test]
    async fn test_update_user() {
        init_env();
        let repo = Repo::new();
        // Create a new user
        let new_user = generate::new_user();
        let user = await! { create_user(repo.clone(), new_user) }.expect("Create user failed.");

        let new_details = UpdateUser {
            bio: Some(fake!(Lorem.paragraph(3, 5)).to_string()),
            image: Some(fake!(Internet.domain_suffix).to_string()),
            email: Some(fake!(Internet.free_email).to_string()),
            ..Default::default()
        };

        // Update the user
        let result = await! { update_user(repo.clone(), user.id, new_details.clone() )};
        result.expect("Failed to update user");

        // Check the user is updated in the database.
        let updated_user = await! {
           find_user(repo.clone(), user.id)
        }
        .expect("Failed to fetch user");
        assert_eq!(updated_user.bio, new_details.bio);
    }

    #[async_test]
    async fn test_list_articles() {
        init_env();
        let repo = Repo::new();

        let test_users = await! {
            (0..5).map(|_| create_user(repo.clone(), generate::new_user()) )
                .collect::<FuturesOrdered<_>>().collect::<Vec<_>>()
        };
        println!("test users {:?}", test_users);

        let articles = await! {
            test_users.into_iter()
                .filter_map(|user_result| user_result.ok())
                .map(|user| create_article(repo.clone(), generate::new_article(user.id)) )
                .collect::<FuturesOrdered<_>>().collect::<Vec<_>>()
        };
        println!("test articles {:?}", articles);
        let results = articles
            .into_iter()
            .filter(|a| a.is_ok())
            .collect::<Vec<_>>();
        assert_eq!(results.len(), 5);
    }
}
