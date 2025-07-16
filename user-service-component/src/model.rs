use spin_sdk::mysql::{Decode, Row};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: String,
    email: String,
    full_name: String,
    gender: String,
    contact: String,
    password: String,
}

pub fn as_user(row: &Row) -> Result<User> {
    let id = String::decode(&row[0])?;
    let email = String::decode(&row[1])?;
    let full_name = String::decode(&row[2])?;
    let gender = String::decode(&row[3])?;
    let contact = String::decode(&row[4])?;
    let password = String::decode(&row[5])?;

    Ok(User {
        id,
        email,
        full_name,
        gender,
        contact,
        password
    })
}

/**** request uer dto ****/

#[derive(Deserialize, Debug)]
pub struct CreateUserModel {
    pub email: String,
    pub full_name: String,
    pub gender: String,
    pub contact: String,
    pub password: String,
}