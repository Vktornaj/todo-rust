extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::tag::{NewTag, Tag};
use crate::models::todo::Todo;
use crate::models::todo_tag::TodoTag;
use crate::models::user::User;
use crate::schema::{_user, _todo, _tag, _todo_tag};


pub fn is_user_tag_existing(
    connection: &mut PgConnection, 
    user_id_: i32, 
    tag_value_: &String
) -> Result<bool, diesel::result::Error> {
    let user = _user::dsl::_user.find(user_id_).first::<User>(connection)?;
    let todos: Vec<i32> = Todo::belonging_to(&user).select(_todo::id).load::<i32>(connection)?;
    Ok(todos.iter().find(
        |&x| 
            is_todo_tag_existing(connection, *x, tag_value_).unwrap_or(false)
    ).is_some())
}

pub fn is_todo_tag_existing(
    connection: &mut PgConnection, 
    todo_id_: i32, 
    tag_value_: &String
) -> Result<bool, diesel::result::Error> {
    let todo = _todo::dsl::_todo.find(todo_id_).first::<Todo>(connection)?;
    let todo_tag_tags: Vec<String> = TodoTag::belonging_to(&todo)
        .inner_join(_tag::table)
        .select(_tag::tag_value)
        .load::<String>(connection)?;
    Ok(todo_tag_tags.iter().find(|&x| x.eq(tag_value_)).is_some())
}

pub fn read_tag(
    connection: &mut PgConnection, 
    user_id_: i32, 
    tag_value_: &String
) -> Result<i32, diesel::result::Error> {
    let tag_id = _todo_tag::dsl::_todo_tag
        .filter(_todo_tag::todo_id.eq_any(
            _todo::table.filter(
                _todo::id.eq_any(
                    _user::table.filter(_user::id.eq(user_id_))
                        .select(_user::id)))
                .select(_todo::id)
        ))
        .filter(_todo_tag::tag_id.eq_any(
            _tag::table.filter(_tag::tag_value.eq(tag_value_)
        ).select(_tag::id)))
        .execute(connection)?;
    Ok(tag_id as i32)
}

pub fn write_tag(
    connection: &mut PgConnection, 
    new_tag: &NewTag
) -> Result<i32, diesel::result::Error> {
    let tag = diesel::insert_into(_tag::table)
        .values(new_tag)
        .get_result::<Tag>(connection)?;
    Ok(tag.id)
}

pub fn associate_todo_tag(
    connection: &mut PgConnection,
    todo_id_: i32,
    tag_id_: i32,
) -> Result<(), diesel::result::Error> {
    let todo_tag_ = TodoTag {
        todo_id: todo_id_, 
        tag_id: tag_id_
    };
    diesel::insert_into(_todo_tag::table)
        .values(&todo_tag_)
        .get_results::<TodoTag>(connection)?;
    Ok(())
}

pub fn read_user_tags(
    connection: &mut PgConnection, 
    user_id_: i32,
) -> Vec<Tag> {
    let user = _user::dsl::_user.find(user_id_).first::<User>(connection);
    if user.is_err() { return Vec::new(); }

    let todos = Todo::belonging_to(user.as_ref().unwrap())
        .load::<Todo>(connection);
    if todos.is_err() { return Vec::new(); }

    todos.unwrap().iter()
        .fold(Vec::new(), |_, x|read_todo_tags(connection, x.id))
}

pub fn read_todo_tags(
    connection: &mut PgConnection, 
    todo_id_: i32,
) -> Vec<Tag> {
    let todo = _todo::dsl::_todo.find(todo_id_).first::<Todo>(connection);
    if todo.is_err() { return Vec::new(); }

    let tags = TodoTag::belonging_to(todo.as_ref().unwrap())
        .inner_join(_tag::table)
        .select(_tag::all_columns)
        .load::<Tag>(connection);

    tags.unwrap_or(Vec::new())
}

pub fn delete_user_tag(
    connection: &mut PgConnection, user_id_: i32, tag_value_: &String
) -> Result<(), diesel::result::Error> {
    diesel::delete(_todo_tag::table)
        .filter(_todo_tag::todo_id.eq_any(
            _todo::table.filter(
                _todo::id.eq_any(
                    _user::table.filter(_user::id.eq(user_id_))
                        .select(_user::id)))
                .select(_todo::id)
        ))
        .filter(_todo_tag::tag_id.eq_any(
            _tag::table.filter(_tag::tag_value.eq(tag_value_)
        ).select(_tag::id)))
        .execute(connection)?;
    Ok(())
}

pub fn delete_todo_tag(
    connection: &mut PgConnection, todo_id_: i32, tag_value_: &String,
) -> Result<(), diesel::result::Error> {
    diesel::delete(_todo_tag::table)
        .filter(_todo_tag::todo_id.eq_any(_todo::table.filter(_todo::id.eq(todo_id_))
            .select(_todo::id)
        ))
        .filter(_todo_tag::tag_id.eq_any(_tag::table.filter(_tag::tag_value.eq(tag_value_))
            .select(_tag::id)))
        .execute(connection)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use std::env;
    use super::*;

    fn establish_connection_pg() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    #[test]
    fn test_is_user_tag_existing() {
        let connection = &mut establish_connection_pg();
        assert_eq!(is_user_tag_existing(connection, user_id_, tag_value_), Ok(true));
    }
}