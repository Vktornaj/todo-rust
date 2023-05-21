use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};
use crate::domain::user::User;


pub trait UserRepository {
    /// Find and return one single record from the persistence system
    fn find_one(&self, username: &String) -> Result<User, RepoSelectError>;

    /// Insert the received entity in the persistence system
    fn create(&self, user: &User) -> Result<User, RepoCreateError>;

    /// Update one single record already present in the persistence system
    fn update(&self, user: &User) -> Result<User, RepoUpdateError>;

    /// Delete one single record from the persistence system
    fn delete(&self, username: &String) -> Result<(), RepoDeleteError>;
}