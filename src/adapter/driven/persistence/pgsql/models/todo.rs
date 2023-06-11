use chrono::{DateTime, Utc};
use diesel::{prelude::*};

use super::super::schema::_todo;
use crate::domain::todo::{Todo as TodoDomain, Status};


#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = _todo)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: u8,
    pub create_date: DateTime<Utc>,
    pub done_date: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn to_domain_todo(self, tags: Vec<String>) -> TodoDomain {
        TodoDomain {
            id: Some(self.id),
            title: self.title,
            description: self.description,
            status: (self.status as i32).try_into().unwrap_or(Status::ABORTED),
            create_date: Some(self.create_date),
            done_date: self.done_date,
            deadline: self.deadline,
            tags,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = _todo)]
pub struct NewTodo {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: i32,
    pub deadline: Option<DateTime<Utc>>,
}

impl NewTodo {
    pub fn from_domain_todo(todo: TodoDomain, user_id: i32) -> Self {
        NewTodo {
            user_id,
            title: todo.title,
            description: todo.description,
            status: todo.status as i32,
            deadline: todo.deadline,
        }
    }
}