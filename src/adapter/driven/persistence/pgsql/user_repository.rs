extern crate diesel;
use diesel::prelude::*;
use rocket::async_trait;

use crate::application::port::driven::user_repository;
use crate::domain::user::User as UserDomain;
use super::models::user::{User as UserDB, NewUser as NewUserDB};
use crate::application::port::driven::errors;
use super::db::Db;
use crate::adapter::driven::persistence::pgsql::schema;


pub struct UserRepository {}

#[async_trait]
impl user_repository::UserRepository<Db> for UserRepository {
    async fn find_one(&self, conn: &Db, username: &String) -> Result<UserDomain, errors::RepoSelectError> {
        use self::schema::_user::dsl::{username as _username, _user};
        let username = username.to_owned();
        match conn.run(move |c| {
            _user.filter(_username.like(username)).first::<UserDB>(c)
        }).await {
            Ok(user_db) => Ok(user_db.to_user_domain()),
            Err(_) => Err(errors::RepoSelectError::NotFound)
        }
        // result.is_err()
    }

    async fn create(&self, conn: &Db, user: UserDomain) -> Result<UserDomain, errors::RepoCreateError> {
        use self::schema::_user::dsl::_user;
        match conn.run(move |c| {
            diesel::insert_into(_user)
                .values(NewUserDB::from_user_domain(user))
                .get_result::<UserDB>(c)
        }).await {
            Ok(user_db) => Ok(user_db.to_user_domain()),
            Err(_) => Err(errors::RepoCreateError::Unknown("db error".to_owned()))
        }
    }

    async fn update(&self, conn: &Db, user: &UserDomain) -> Result<UserDomain, errors::RepoUpdateError> {
        todo!()
    }

    async fn delete(&self, conn: &Db, username: &String) -> Result<(), errors::RepoDeleteError> {
        todo!()
    }
}