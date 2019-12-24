use crate::db;
use crate::models::{NewUser, UpdateUser, User};
use crate::schema::users;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

type Repo = db::Repo<PgConnection>;

pub fn insert(repo: &Repo, user: NewUser) -> Result<User, Error> {
    repo.run(move |conn| {
        // TODO: store password not in plain text, later
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
    })
}

pub fn find(repo: &Repo, user_id: i32) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    repo.run(move |conn| users.find(user_id).first(&conn))
}

pub fn find_by_email_password(
    repo: &Repo,
    user_email: String,
    user_password: String,
) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    repo.run(|conn| {
        users
            .filter(email.eq(user_email))
            .filter(password.eq(user_password))
            .first::<User>(&conn)
    })
}

pub fn update(repo: &Repo, user_id: i32, details: UpdateUser) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    repo.run(move |conn| {
        diesel::update(users.find(user_id))
            .set(&details)
            .get_result(&conn)
    })
}

#[cfg(test)]
mod tests {

    // use tokio_async_await_test::async_test;

    // #[async_test]
    // async fn test_create_user() {
    //     let repo = Repo::new();

    //     let new_user = generate::new_user();
    //     let user =  insert(repo.clone(), new_user).await.expect("Create user failed.");

    //     let results =
    //        find(repo.clone(), user.id).await
    //     ;
    //     assert!(results.is_ok());
    // }

    // #[async_test]
    // async fn test_authenticate_user() {
    //     let repo = Repo::new();
    //     // Create a new user
    //     let new_user = generate::new_user();
    //     let user =  insert(repo.clone(), new_user).await.expect("Create user failed.");

    //     // Check the user is in the database.
    //     let results =
    //        find_by_email_password(repo.clone(), user.email, user.password).await
    //     ;
    //     assert!(results.is_ok());
    // }

    // #[async_test]
    // async fn test_update_user() {
    //     let repo = Repo::new();
    //     // Create a new user
    //     let new_user = generate::new_user();
    //     let user =  insert(repo.clone(), new_user).await.expect("Create user failed.");

    //     let new_details = UpdateUser {
    //         bio: Some(fake!(Lorem.paragraph(3, 5)).to_string()),
    //         image: Some(fake!(Internet.domain_suffix).to_string()),
    //         email: Some(fake!(Internet.free_email).to_string()),
    //         ..Default::default()
    //     };

    //     // Update the user
    //     let result =  update(repo.clone(), user.id, new_details.clone() ).await;
    //     result.expect("Failed to update user");

    //     // Check the user is updated in the database.
    //     let updated_user =
    //        find(repo.clone(), user.id).await

    //     .expect("Failed to fetch user");
    //     assert_eq!(updated_user.bio, new_details.bio);
    // }
}
