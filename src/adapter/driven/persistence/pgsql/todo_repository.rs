extern crate diesel;
use diesel::{prelude::*};
use async_trait::async_trait;

use crate::application::port::driven::{todo_repository, errors};
use crate::domain::todo::Todo as TodoDomain;
use super::db::Db;


pub struct TodoRepository {}

#[async_trait]
impl todo_repository::TodoRepository<Db> for TodoRepository {
    async fn find_one(&self, conn: &Db,  id: i32) -> Result<TodoDomain, errors::RepoSelectError> {
        todo!()
    }

    async fn find_all(
        &self, 
        conn: &Db, 
        username: &String, 
        from: i32, 
        to: i32
    ) -> Result<Vec<TodoDomain>, errors::RepoFindAllError> {
        todo!()
    }

    async fn find_one_criteria(
        &self, 
        conn: &Db, 
        username: &String, 
        find_todo: &todo_repository::FindTodo
    ) -> Result<TodoDomain, errors::RepoSelectError> {
        todo!()
    }

    async fn find_all_criteria(
        &self, conn: &Db, 
        username: &String,
        from: i32, 
        to: i32, 
        find_todo: &todo_repository::FindTodo
    ) -> Result<Vec<TodoDomain>, errors::RepoFindAllError> {
        todo!()
    }

    async fn find_all_date_range(
        &self, conn: &Db, 
        username: &String, 
        from: i32, 
        to: i32, 
        find_todo_by_date_range: todo_repository::FindTodoByDateRange
    ) -> Result<Vec<TodoDomain>, errors::RepoFindAllError> {
        todo!()
    }

    async fn create(
        &self, 
        conn: &Db, 
        username: &String, 
        todo: TodoDomain
    ) -> Result<TodoDomain, errors::RepoCreateError> {
        todo!()
    }

    async fn update(
        &self, 
        conn: &Db, 
        username: &String, 
        todo: todo_repository::UpdateTodo
    ) -> Result<TodoDomain, errors::RepoUpdateError> {
        todo!()
    }

    async fn add_tag(
        &self, 
        conn: &Db, 
        username: &String, 
        todo_id: i32, 
        tag: &String
    ) -> Result<TodoDomain, errors::RepoUpdateError> {
        todo!()
    }

    async fn remove_tag(
        &self, 
        conn: &Db, 
        username: &String, 
        todo_id: i32, 
        tag: &String
    ) -> Result<TodoDomain, errors::RepoUpdateError> {
        todo!()
    }

    async fn delete(&self, conn: &Db, username: &String, id: i32) -> Result<TodoDomain, errors::RepoDeleteError> {
        todo!()
    }

    async fn delete_all_criteria(
        &self, conn: &Db, 
        username: &String, 
        find_todo: &todo_repository::FindTodo
    ) -> Result<Vec<TodoDomain>, crate::application::port::driven::errors::RepoDeleteError> {
        todo!()
    }
}