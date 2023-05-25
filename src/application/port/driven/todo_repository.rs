use chrono::{DateTime, Utc};

use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoFindAllError, 
    RepoSelectError, 
    RepoUpdateError
};
use crate::domain::todo::{Todo, Status};


pub struct FindTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub tags: Option<Vec<String>>,
}

pub struct FindTodoByDateRange {
    pub create_date: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub done_date: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub deadline: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub create_date: DateTime<Utc>,
    pub done_date: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
}

pub trait TodoRepository {
    /// Find and return one single record from the persistence system
    fn find_one(&self, id: i64) -> Result<Todo, RepoSelectError>;

    /// Find and return all records corresponding to the user
    fn find_all(&self, username: &String, from: i64, to: i64) -> Result<Vec<Todo>, RepoFindAllError>;
    
    /// Find and return one single record from the persistence system
    fn find_one_criteria(&self, username: &String, find_todo: &FindTodo) -> Result<Todo, RepoSelectError>;

    /// Find and return all records corresponding to the search criteria from the persistence system
    fn find_all_criteria(
        &self, 
        username: &String,
        from: i64, 
        to: i64, 
        find_todo: &FindTodo
    ) -> Result<Vec<Todo>, RepoFindAllError>;
    
    /// Find and return all records corresponding to the search criteria from the persistence system
    fn find_all_date_range(
        &self, 
        username: &String, 
        from: i64, 
        to: i64, 
        find_todo_by_date_range: FindTodoByDateRange
    ) -> Result<Vec<Todo>, RepoFindAllError>;

    /// Insert the received entity in the persistence system
    fn create(&self, username: &String, todo: &Todo) -> Result<Todo, RepoCreateError>;

    /// Update one single record already present in the persistence system
    fn update(&self, username: &String, todo: &UpdateTodo) -> Result<Todo, RepoUpdateError>;
    
    /// Update one single record already present in the persistence system
    fn add_tag(&self, username: &String, todo_id: i64, tag: &String) -> Result<Todo, RepoUpdateError>;
    
    /// Update one single record already present in the persistence system
    fn remove_tag(&self, username: &String, todo_id: i64, tag: &String) -> Result<Todo, RepoUpdateError>;

    /// Delete one single record from the persistence system
    fn delete(&self, username: &String, id: i64) -> Result<(), RepoDeleteError>;
    
    /// Delete one single record from the persistence system
    fn delete_all_criteria(
        &self, 
        username: &String, 
        find_todo: &FindTodo
    ) -> Result<(), RepoDeleteError>;
}