use anyhow::{anyhow, Context, Result};
use spin_sdk::{
    pg::{Connection, ParameterValue},
    variables,
};
use uuid::Uuid;
use crate::model::{as_image, CreateImageModel, Image};

const VAR_CONNECTION_STRING: &str = "db_connection_string_pgsql";
const SQL_CREATE_IMAGE: &str = "INSERT INTO images (id,alt,src) VALUES ($1,$2,$3)";
const SQL_SELECT_ALL_IMAGES: &str = "SELECT * FROM images";
const SQL_SELECT_IMAGE: &str = "SELECT * FROM images WHERE id=$1";
const SQL_DELETE_IMAGE: &str = "DELETE FROM images WHERE id=$1";
const SQL_UPDATE_IMAGE: &str = "UPDATE images SET alt=$1,src=$2 WHERE id=$3";

/****  pg sql configuration ****/
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
        .with_context(|| "Could not establish connection to PostgreSQL  database")
}

/****  Image CRUD ****/

pub fn create_image(dto: &CreateImageModel) -> Result<()> {
    let conn = get_connection()?;
    let id = Uuid::new_v4();

    let params = vec![
        ParameterValue::Str(id.to_string()),
        ParameterValue::Str(dto.alt.clone()),
        ParameterValue::Str(dto.src.clone()),
    ];

    conn.execute(SQL_CREATE_IMAGE, &params)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}

pub fn get_all_images() -> Result<Vec<Image>> {
    let conn = get_connection()?;
    let row_set = conn.query(SQL_SELECT_ALL_IMAGES, &[])
        .with_context(|| "Error while executing statement on database")?;

    let images = row_set
        .rows
        .iter()
        .map(|row| as_image(row))
        .collect::<Result<Vec<_>>>()?;

    Ok(images)
}

pub fn get_image_by_id(id: &str) -> Result<Image> {
    let conn = get_connection()?;
    let param = vec![ParameterValue::Str(id.to_string())];
    let row_set = conn.query(SQL_SELECT_IMAGE, &param)
        .with_context(|| "Error while executing statement on database")?;

    let row = row_set.rows.first().ok_or_else(||anyhow!("Image not found"))?;
    let image = as_image(row)?;
    Ok(image)
}

pub fn delete_image_by_id(id: &str) -> Result<()> {
    let conn = get_connection()?;
    let param = vec![ParameterValue::Str(id.to_string())];
    conn.execute(SQL_DELETE_IMAGE, &param)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}

pub fn update_image_by_id(id: &str, dto: CreateImageModel) -> Result<()> {
    let conn = get_connection()?;
    let param = vec![ParameterValue::Str(id.to_string())];
    let row_set = conn.query(SQL_SELECT_IMAGE, &param)
        .with_context(|| "Error while executing statement on database")?;
    row_set.rows.first().ok_or_else(||anyhow!("Image not found"))?;

    let params = vec![
        ParameterValue::Str(dto.alt.clone()),
        ParameterValue::Str(dto.src.clone()),
        ParameterValue::Str(id.to_string()),
    ];

    conn.execute(SQL_UPDATE_IMAGE, &params)
        .with_context(|| "Error while executing statement on database")
        .and_then(|_| Ok(()))
}