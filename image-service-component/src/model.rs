use serde::{Deserialize, Serialize};
use spin_sdk::pg::{Row, Decode};
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    id: String,
    alt: String,
    src: String,
}

pub fn as_image(row: &Row) -> Result<Image> {
    let id = String::decode(&row[0])?;
    let alt = String::decode(&row[1])?;
    let src = String::decode(&row[2])?;

    Ok(Image {
        id,
        alt,
        src,
    })
}

/**** request image dto ****/

#[derive(Deserialize, Debug)]
pub struct CreateImageModel {
    pub alt: String,
    pub src: String,
}