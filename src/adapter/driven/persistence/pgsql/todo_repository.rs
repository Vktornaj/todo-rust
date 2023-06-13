extern crate diesel;
use chrono::{DateTime, Utc};
use diesel::{prelude::*};
use async_trait::async_trait;
use diesel::sql_types::{Nullable, Int4, Array, VarChar};

use crate::application::port::driven::{todo_repository, errors};
use crate::domain::todo::Todo as TodoDomain;
use super::models::todo::{Todo as TodoDB, NewTodo as NewTodoDB};
use super::db::Db;
use crate::adapter::driven::persistence::pgsql::schema;
use self::schema::_todo::dsl::{
    _todo,
};


// Postgres functions
// TODO: Improve
sql_function! { 
    fn find_todo_sql(
        p_username: VarChar, p_title: Nullable<VarChar>, p_description: Nullable<VarChar>, p_status: Nullable<Int4>, p_tags: Nullable<Array<VarChar>>,
    ) -> Record<(
        Int4, VarChar, VarChar, Nullable<VarChar>, Int4, Timestamptz, Nullable<Timestamptz>, Nullable<Timestamptz>,
    )>;
}

// TODO: Write PlPgSQL function
sql_function! { 
    fn find_tags_sql(todo_id: Int4) -> Array<VarChar>;
}

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
        find_todo: todo_repository::FindTodo
    ) -> Result<TodoDomain, errors::RepoSelectError> {
        match conn.run(move |c| {
            let res = diesel::select(find_todo_sql(
                find_todo.username,
                find_todo.title,
                find_todo.description,
                find_todo.status.and_then(|x| Some(x as i32)),
                find_todo.tags
            )).get_result::<(
                i32,
                String, 
                String, 
                Option<String>, 
                i32, 
                DateTime<Utc>, 
                Option<DateTime<Utc>>, 
                Option<DateTime<Utc>>
            )>(c);
            match res {
                Ok(
                    todo_tuple
                ) => match diesel::select(find_tags_sql(todo_tuple.0)).get_result::<Vec<String>>(c) {
                    Ok(tags) => Ok((todo_tuple, tags)),
                    Err(_) => Err(errors::RepoSelectError::NotFound),
                },
                Err(_) => Err(errors::RepoSelectError::Unknown("".to_owned())),
            }
        }).await {
            Ok((
                todo_tuple, 
                tags
            )) => Ok(TodoDB::from_tuple(todo_tuple).to_domain_todo(tags)),
            Err(err) => Err(err)
        }
    }

    async fn find_all_criteria(
        &self, conn: &Db, 
        username: &String,
        from: i32, 
        to: i32, 
        find_todo: todo_repository::FindTodo
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
        let tags = todo.tags.clone();
        let username = username.to_owned();
        // TODO: Create tags
        match conn.run(move |c| {
            diesel::insert_into(_todo)
                .values(NewTodoDB::from_domain_todo(todo, &username))
                .get_result::<TodoDB>(c)
        }).await {
            Ok(user_db) => Ok(user_db.to_domain_todo(tags)),
            Err(_) => Err(errors::RepoCreateError::Unknown("db error".to_owned()))
        }
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
        find_todo: todo_repository::FindTodo
    ) -> Result<Vec<TodoDomain>, crate::application::port::driven::errors::RepoDeleteError> {
        todo!()
    }
}