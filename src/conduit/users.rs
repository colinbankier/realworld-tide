use super::errors::{FindError, InternalError};
use crate::db::models::{NewUser, UpdateUser, User};
use crate::db::schema::users;
use bcrypt::BcryptResult;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use uuid::Uuid;

type Repo = crate::db::Repo<PgConnection>;

pub fn insert(repo: &Repo, mut user: NewUser) -> Result<User, InternalError> {
    user.password = hash(user.password)?;

    let user = repo.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
    })?;

    Ok(user)
}

pub fn find(repo: &Repo, user_id: Uuid) -> Result<User, InternalError> {
    use crate::db::schema::users::dsl::*;
    let user = repo.run(move |conn| users.find(user_id).first(&conn))?;

    Ok(user)
}

pub fn find_by_username(repo: &Repo, username_value: String) -> Result<User, DieselError> {
    use crate::db::schema::users::dsl::*;
    repo.run(move |conn| users.filter(username.eq(username_value)).first(&conn))
}

pub fn find_by_email_password(
    repo: &Repo,
    user_email: String,
    user_password: String,
) -> Result<User, FindError> {
    use crate::db::schema::users::dsl::*;
    let user = repo.run(|conn| users.filter(email.eq(user_email)).first::<User>(&conn))?;
    verify(user_password, user)
}

pub fn update(repo: &Repo, user_id: Uuid, mut details: UpdateUser) -> Result<User, InternalError> {
    use crate::db::schema::users::dsl::*;

    details.password = details.password.map(hash).transpose()?;

    let user = repo.run(move |conn| {
        diesel::update(users.find(user_id))
            .set(&details)
            .get_result(&conn)
    })?;

    Ok(user)
}

fn hash<P: AsRef<[u8]>>(password: P) -> BcryptResult<String> {
    // TODO read hash cost from configuration, defaulting to bcrypt::DEFAULT_COST
    // (4 is the minimum for use in tests)
    bcrypt::hash(password, 4)
}

fn verify(password: String, user: User) -> Result<User, FindError> {
    if bcrypt::verify(password, &user.password)? {
        Ok(user)
    } else {
        Err(FindError::NotFound)
    }
}
