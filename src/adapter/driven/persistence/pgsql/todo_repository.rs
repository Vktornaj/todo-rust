use crate::application::port::driven::todo_repository;


struct TodoRepository {}

impl todo_repository::TodoRepository for TodoRepository {
    fn find_one(&self, id: i64) -> Result<crate::domain::todo::Todo, crate::application::port::driven::errors::RepoSelectError> {
        todo!()
    }

    fn find_all(&self, username: &String, from: i64, to: i64) -> Result<Vec<crate::domain::todo::Todo>, crate::application::port::driven::errors::RepoFindAllError> {
        todo!()
    }

    fn find_one_criteria(&self, username: &String, find_todo: &todo_repository::FindTodo) -> Result<crate::domain::todo::Todo, crate::application::port::driven::errors::RepoSelectError> {
        todo!()
    }

    fn find_all_criteria(
        &self, 
        username: &String,
        from: i64, 
        to: i64, 
        find_todo: &todo_repository::FindTodo
    ) -> Result<Vec<crate::domain::todo::Todo>, crate::application::port::driven::errors::RepoFindAllError> {
        todo!()
    }

    fn find_all_date_range(
        &self, 
        username: &String, 
        from: i64, 
        to: i64, 
        find_todo_by_date_range: todo_repository::FindTodoByDateRange
    ) -> Result<Vec<crate::domain::todo::Todo>, crate::application::port::driven::errors::RepoFindAllError> {
        todo!()
    }

    fn create(&self, username: &String, todo: &crate::domain::todo::Todo) -> Result<crate::domain::todo::Todo, crate::application::port::driven::errors::RepoCreateError> {
        todo!()
    }

    fn update(&self, username: &String, todo: &todo_repository::UpdateTodo) -> Result<crate::domain::todo::Todo, crate::application::port::driven::errors::RepoUpdateError> {
        todo!()
    }

    fn add_tag(&self, username: &String, todo_id: i64, tag: &String) -> Result<crate::domain::todo::Todo, crate::application::port::driven::errors::RepoUpdateError> {
        todo!()
    }

    fn remove_tag(&self, username: &String, todo_id: i64, tag: &String) -> Result<crate::domain::todo::Todo, crate::application::port::driven::errors::RepoUpdateError> {
        todo!()
    }

    fn delete(&self, username: &String, id: i64) -> Result<(), crate::application::port::driven::errors::RepoDeleteError> {
        todo!()
    }

    fn delete_all_criteria(
        &self, 
        username: &String, 
        find_todo: &todo_repository::FindTodo
    ) -> Result<(), crate::application::port::driven::errors::RepoDeleteError> {
        todo!()
    }
}