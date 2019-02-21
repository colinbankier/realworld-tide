use crate::db::Repo;
use crate::models::User;
use crate::models::*;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::users;
    use crate::schema::users::dsl::*;
    use crate::test_helpers::generate;
    use crate::test_helpers::init_env;
    use diesel::prelude::*;
    use fake::fake;
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

        // Check the user is updated in the database.
        let updated_user = await! {
           find_user(repo.clone(), user.id)
        }
        .expect("Failed to fetch user");
        assert_eq!(updated_user.bio, new_details.bio);
    }
}
