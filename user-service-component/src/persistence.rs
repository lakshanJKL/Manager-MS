use anyhow::{anyhow, Context, Error, Result};
use spin_sdk::{
    variables,
    mysql::{Connection, ParameterValue},
};
use uuid::Uuid;
use crate::model::{as_user, CreateUserModel, User};

const VAR_CONNECTION_STRING: &str = "db_connection_string";
const SQL_CREATE_USER: &str = "INSERT INTO users (id,email,full_name,gender,contact,password) VALUES (?,?,?,?,?,?)";
const SQL_SELECT_USER_BY_EMAIL: &str = "SELECT * FROM users WHERE email=?";
const SQL_DELETE_USER_BY_ID: &str = "DELETE FROM users WHERE id=?";

/**** MySQL configurations ****/
fn get_connection_string() -> Result<String> {
    variables::get(VAR_CONNECTION_STRING).with_context(|| {
        format!(
            "Connection String not specified please set {}",
            VAR_CONNECTION_STRING
        )
    })
}

fn get_connection() -> Result<Connection> {
    let connection_string = get_connection_string()?;
    Connection::open(&*connection_string)
        .with_context(|| "Could not establish connection to MySQL database")
}

/**** user create ****/

pub fn create_user(dto: &CreateUserModel) -> Result<()> {
    let conn = get_connection()?;
    let id = Uuid::new_v4();

    let params = vec![
        ParameterValue::Str(id.to_string()),
        ParameterValue::Str(dto.email.clone()),
        ParameterValue::Str(dto.full_name.clone()),
        ParameterValue::Str(dto.gender.clone()),
        ParameterValue::Str(dto.contact.clone()),
        ParameterValue::Str(dto.password.clone()),
    ];

    conn.execute(SQL_CREATE_USER, &params)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}

pub fn delete_user_by_id(id: &str) -> Result<()> {
    let conn = get_connection()?;
    let param = vec![ParameterValue::Str(id.to_string())];

    conn.execute(SQL_DELETE_USER_BY_ID, &param)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}

pub fn find_user_by_email(email: &str) -> Result<Option<User>> {
    let conn = get_connection()?;
    let param = vec![ParameterValue::Str(email.to_string())];
    let row_set = conn.query(SQL_SELECT_USER_BY_EMAIL, &param)?;

    if let Some(row) = row_set.rows.first() {
        let user = as_user(row)?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
