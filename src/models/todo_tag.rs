use diesel::{prelude::*};

use super::super::schema::_todo_tag;
use crate::models::todo::Todo;


#[derive(Identifiable, Associations, Queryable, PartialEq, Debug, Insertable)]
#[diesel(primary_key(todo_id, tag_id))]
#[diesel(belongs_to(Todo))]
#[diesel(table_name = _todo_tag)]
pub struct TodoTag {
    pub todo_id: i32,
    pub tag_id: i32,
}