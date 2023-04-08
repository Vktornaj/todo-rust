extern crate diesel;

use diesel::pg::PgConnection;
use diesel::{prelude::*};

use crate::models::todo::{Todo, NewTodo, TodoUpdate};
use crate::models::user::User;
use crate::schema;
use crate::database;


pub fn is_available_title(connection: &mut PgConnection, user_id_: i32, title_: &String) -> bool {
    use self::schema::_todo::dsl::*;
    let user = database::user::read_user(connection, &user_id_);
    if user.is_none() {
        return false;
    }
    Todo::belonging_to(&user.unwrap())
        .filter(title.like(title_))
        .first::<Todo>(connection)
        .is_err()
}

pub fn write_todo(connection: &mut PgConnection, new_todo: NewTodo<'_>) {
    use self::schema::_todo::dsl::*;
    diesel::insert_into(_todo)
        .values(&new_todo)
        .execute(connection)
        .expect("Error saving new user");
}

pub fn read_todos(
    connection: &mut PgConnection, 
    user: &User,
    from: i64,
    to: i64,
) -> Vec<Todo> {
    use self::schema::_todo::dsl::*;
    Todo::belonging_to(user)
        .order(id)
        .limit(to)
        .offset(from)
        .load::<Todo>(connection)
        .expect("Error loading todos")
}

pub fn is_belonging_to(connection: &mut PgConnection, user_id_: i32, todo_id: i32) -> bool {
    let user = database::user::read_user(connection, &user_id_);
    if user.is_none() {
        return false;
    }

    Todo::belonging_to(&user.unwrap())
        .find(todo_id)
        .first::<Todo>(connection)
        .is_ok()
}

// TODO: to optimize
pub fn update_todo(connection: &mut PgConnection, todo_update: TodoUpdate) -> Result<(), diesel::result::Error> {
    use self::schema::_todo::dsl::*;

    let update_statement = diesel::update(_todo.find(todo_update.id));
    if todo_update.title.is_some() {
        update_statement.clone().set(title.eq(todo_update.title.unwrap())).execute(connection)?;
    }
    if todo_update.description.is_some() {
        update_statement.clone().set(description.eq(todo_update.description.unwrap())).execute(connection)?;
    }
    if todo_update.status.is_some() {
        update_statement.clone().set(status.eq(todo_update.status.unwrap())).execute(connection)?;
    }
    if todo_update.done_date.is_some() {
        update_statement.clone().set(done_date.eq(todo_update.done_date.unwrap())).execute(connection)?;
    }
    if todo_update.deadline.is_some() {
        update_statement.set(deadline.eq(todo_update.deadline.unwrap())).execute(connection)?;
    }

    Ok(())
}

pub fn delete_todo(connection: &mut PgConnection, id_: i32) -> Result<(), &'static str> {
    use self::schema::_todo::dsl::*;
    diesel::delete(_todo.find(id_))
        .execute(connection)
        .expect("Error deleting todo");
    Ok(())
}