use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::SystemTime;

use super::user::User;
use super::super::schema::_todo;


// pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[derive(Deserialize, Serialize)]
pub enum Status {
    PENDING,
    STARTED,
    DONE,
    PAUSED,
    ABORTED,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = _todo)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: i32,
    pub create_date: SystemTime,
    pub done_date: Option<SystemTime>,
    pub deadline: Option<SystemTime>,
    // pub tags: Vec<String>,
}

// impl Todo {
//     pub fn attach(self) -> TodoJson {
//         TodoJson {
//             id: self.id,
//             title: self.title,
//             description: self.description,
//             status: self.status,
//             create_date: self.create_date.format(DATE_FORMAT).to_string(),
//             done_date: Some(self.done_date.unwrap().format(DATE_FORMAT).to_string()),
//             deadline: Some(self.deadline.unwrap().format(DATE_FORMAT).to_string()),
//             tags: self.tags,
//         }
//     }
// }

// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TodoJson {
//     pub id: String,
//     pub title: String,
//     pub description: String,
//     pub status: Status,
//     pub create_date: String,
//     pub done_date: Option<String>,
//     pub deadline: Option<String>,
//     pub tags: Vec<String>,
// }