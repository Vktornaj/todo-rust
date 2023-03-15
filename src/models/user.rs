use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

use super::super::schema::_user;


#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = _user)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
}

impl User {
    pub fn attach(self) -> UserJson {
        UserJson {
            username: self.username,
            first_name: self.first_name.unwrap_or("".to_string()),
            last_name: self.last_name.unwrap_or("".to_string()),
            password: self.password,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = _user)]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub first_name: &'a String,
    pub last_name: &'a String,
    pub password: &'a String,
}

impl NewUser<'_> {
    pub fn attach(self) -> NewUserJson {
        NewUserJson {
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserJson {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl NewUserJson {
    pub fn attach<'a>(&'a self) -> NewUser<'a> {
        NewUser {
            username: &self.username,
            first_name: &self.first_name,
            last_name: &self.last_name,
            password: &self.password,
        }
    }
}