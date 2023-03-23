use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error
    },
    Argon2
};

use super::super::schema::_user;


#[derive(Identifiable, Queryable, PartialEq, Debug)]
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
        }
    }

    // TODO: Reduce the runtime; 1.2 seconds
    pub fn verify_password(&self, password: &String) -> Result<(), Error> {
        let parsed_hash = PasswordHash::new(&self.password)?;
        Argon2::default().verify_password(
            password.as_bytes(), 
            &parsed_hash
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = _user)]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl NewUser {
    pub fn attach(self) -> NewUserJson {
        NewUserJson {
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            password: self.password,
        }
    }
    
    // TODO: Reduce the runtime; 1.3 seconds
    pub fn hash_password(&mut self) -> Result<(), Error>{
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        self.password = argon2.hash_password(self.password.as_bytes(), &salt)?
            .to_string();
        Ok(())
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
    pub fn attach(self) -> NewUser {
        NewUser {
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            password: self.password,
        }
    }
}