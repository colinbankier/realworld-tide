use crate::db;
use crate::models::{NewUser, UpdateUser, User};
use crate::schema::users;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

type Repo = db::Repo<PgConnection>;

pub fn insert(repo: &Repo, user: NewUser) -> Result<User, Error> {
    repo.run(move |conn| {
        // TODO: store password not in plain text, later
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
    })
}

pub fn find(repo: &Repo, user_id: Uuid) -> Result<User, Error> {
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

pub fn update(repo: &Repo, user_id: Uuid, details: UpdateUser) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    repo.run(move |conn| {
        diesel::update(users.find(user_id))
            .set(&details)
            .get_result(&conn)
    })
}
