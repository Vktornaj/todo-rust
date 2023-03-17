extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::user::{User, NewUser};
use crate::schema;


pub fn is_available_username(connection: &mut PgConnection, username_: &String) -> bool {
    use self::schema::_user::dsl::*;
    let result = _user
        .filter(username.like(username_))
        .limit(1)
        .load::<User>(connection)
        .expect("Error loading user");
    result.len() == 0
}

pub fn write_user(connection: &mut PgConnection, new_user: &NewUser) {
    use self::schema::_user::dsl::*;
    diesel::insert_into(_user)
        .values(new_user)
        .execute(connection)
        .expect("Error saving new user");
}

pub fn read_users(connection: &mut PgConnection) -> Vec<User> {
    use self::schema::_user::dsl::*;
    _user
        .load::<User>(connection)
        .expect("Error loading users")
}

pub fn read_user(connection: &mut PgConnection, username_: &String) -> Option<User> {
    use self::schema::_user::dsl::*;
    let mut users = _user
        .filter(username.eq(username_))
        .limit(1)
        .load::<User>(connection)
        .expect("Error reading user");
    if users.len() > 0 {
        Some(users.remove(0))
    } else {
        None
    }
}