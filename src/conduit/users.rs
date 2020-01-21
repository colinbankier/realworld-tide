use crate::db::models::{NewUser, UpdateUser, User};
use crate::db::schema::users;
use crate::{domain, Repo};

use crate::conduit::followers::follow;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn insert(repo: &Repo, user: NewUser) -> Result<User, Error> {
    let new_user: User = repo.run(move |conn| {
        // TODO: store password not in plain text, later
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
    })?;

    // Invariant: a user always follows themselves
    follow(repo, new_user.id, new_user.id)?;

    Ok(new_user)
}

pub fn find(repo: &Repo, user_id: Uuid) -> Result<User, domain::GetUserError> {
    use crate::db::schema::users::dsl::*;
    let result = repo.run(move |conn| users.find(user_id).first(&conn));
    result.map_err(|e| match e {
        e @ Error::NotFound => domain::GetUserError::NotFound { user_id, source: e },
        e => domain::GetUserError::DatabaseError(e),
    })
}

pub fn find_by_username(repo: &Repo, username_value: &str) -> Result<User, Error> {
    use crate::db::schema::users::dsl::*;
    repo.run(move |conn| users.filter(username.eq(username_value)).first(&conn))
}

pub fn find_by_email_password(
    repo: &Repo,
    user_email: String,
    user_password: String,
) -> Result<User, Error> {
    use crate::db::schema::users::dsl::*;
    repo.run(|conn| {
        users
            .filter(email.eq(user_email))
            .filter(password.eq(user_password))
            .first::<User>(&conn)
    })
}

pub fn update(repo: &Repo, user_id: Uuid, details: UpdateUser) -> Result<User, Error> {
    use crate::db::schema::users::dsl::*;
    repo.run(move |conn| {
        diesel::update(users.find(user_id))
            .set(&details)
            .get_result(&conn)
    })
}
