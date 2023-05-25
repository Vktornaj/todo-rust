use rocket_sync_db_pools::database;

use crate::application::port::driven::user_repository;

#[database("my_db")]
pub struct Db(diesel::PgConnection);
pub struct UserRepository {
    conn: Db
}

impl UserRepository {
    pub fn new() -> Self {
        todo!()
    }
}

impl user_repository::UserRepository for UserRepository {
    fn find_one(&self, username: &String) -> Result<crate::domain::user::User, crate::application::port::driven::errors::RepoSelectError> {
        todo!()
    }

    fn create(&self, user: &crate::domain::user::User) -> Result<crate::domain::user::User, crate::application::port::driven::errors::RepoCreateError> {
        todo!()
    }

    fn update(&self, user: &crate::domain::user::User) -> Result<crate::domain::user::User, crate::application::port::driven::errors::RepoUpdateError> {
        todo!()
    }

    fn delete(&self, username: &String) -> Result<(), crate::application::port::driven::errors::RepoDeleteError> {
        todo!()
    }
}