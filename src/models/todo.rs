use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::convert::TryFrom;

use super::user::User;
use super::super::schema::_todo;


pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[derive(Deserialize, Serialize)]
pub enum Status {
    PENDING,
    STARTED,
    DONE,
    PAUSED,
    ABORTED,
}

impl TryFrom<i32> for Status {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Status::PENDING as i32 => Ok(Status::PENDING),
            x if x == Status::STARTED as i32 => Ok(Status::STARTED),
            x if x == Status::DONE as i32 => Ok(Status::DONE),
            x if x == Status::PAUSED as i32 => Ok(Status::PAUSED),
            x if x == Status::ABORTED as i32 => Ok(Status::ABORTED),
            _ => Err(()),
        }
    }
}

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = _todo)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: i32,
    pub create_date: DateTime<Utc>,
    pub done_date: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn attach(self) -> TodoJson {
        TodoJson {
            id: self.id,
            title: self.title,
            description: self.description.unwrap_or("".to_string()),
            status: self.status.try_into().unwrap_or(Status::ABORTED),
            create_date: self.create_date.format(DATE_FORMAT).to_string(),
            done_date: match self.done_date {
                Some(date) => date.format(DATE_FORMAT).to_string(),
                None => "".to_string(),
            },
            deadline: match self.deadline {
                Some(date) => date.format(DATE_FORMAT).to_string(),
                None => "".to_string(),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoJson {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub create_date: String,
    pub done_date: String,
    pub deadline: String,
    // pub tags: Vec<String>,
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = _todo)]
pub struct NewTodo<'a> {
    pub user_id: i32,
    pub title: &'a String,
    pub description: &'a String,
    pub status: &'a i32,
}

impl NewTodo<'_> {
    pub fn attach(self) -> NewTodoJson {
        NewTodoJson {
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTodoJson {
    pub title: String,
    pub description: String,
    pub status: i32,
}

impl NewTodoJson {
    pub fn attach<'a>(&'a self, user_id: i32) -> NewTodo<'a> {
        NewTodo {
            user_id: user_id,
            title: &self.title,
            description: &self.description,
            status: &self.status,
        }
    }
}