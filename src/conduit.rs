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

#[cfg(test)]
mod tests {
    use tokio_async_await_test::async_test;
    use crate::test_helpers::init_env;
    use super::*;
    use crate::schema::users;
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use fake::fake;

    #[async_test]
    async fn test_create_user() {
        init_env();
        let repo = Repo::new();
        // Create a new user
        let new_user = NewUser {
                username: fake!(Internet.user_name).to_string(),
                email: fake!(Internet.free_email).to_string(),
                password: fake!(Lorem.word).to_string(),
            };
        let user = await!{ create_user(repo.clone(), new_user) };

        // Check the user is in the database.
        let user_id = user.expect("Create user failed.").id;
        let results = await! {
            find_user(repo.clone(), user_id)
         };
        assert!(results.is_ok());
    }
}

