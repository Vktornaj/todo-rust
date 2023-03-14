use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::super::schema::_user;


#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = _user)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
}