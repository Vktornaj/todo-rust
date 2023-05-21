use crate::domain::user::User;

use super::super::port::driven::user_repository::UserRepository;


pub fn execute(repo: &impl UserRepository, username: &String) -> Option<User> {
    repo.find_one(username).ok()
}