use super::super::port::driven::user_repository::UserRepository;


pub fn execute(repo: &impl UserRepository, username: &String) -> bool {
    repo.find_one(username).is_ok()
}