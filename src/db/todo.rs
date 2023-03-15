extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::todo::{Todo, NewTodo};
use crate::schema;


pub fn is_available_title(connection: &mut PgConnection, title_: &String) -> bool {
    use self::schema::_todo::dsl::*;
    let result = _todo
        .filter(title.like(title_))
        .limit(1)
        .load::<Todo>(connection)
        .expect("Error loading user");
    result.len() == 0
}

pub fn write_todo(connection: &mut PgConnection, new_todo: NewTodo<'_>) {
    use self::schema::_todo::dsl::*;
    diesel::insert_into(_todo)
        .values(&new_todo)
        .execute(connection)
        .expect("Error saving new user");
}

pub fn read_todos(connection: &mut PgConnection) -> Vec<Todo> {
    use self::schema::_todo::dsl::*;
    _todo
        .load::<Todo>(connection)
        .expect("Error loading users")
}