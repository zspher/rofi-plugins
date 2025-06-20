use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

use serde::Deserialize;
use serde_json::from_str;

pub fn get_engine_list() -> Result<HashMap<String, Search>, io::Error> {
    let data_path = dirs::data_dir().ok_or(io::Error::from(io::ErrorKind::NotFound))?;

    let mut s = String::new();
    File::open(data_path.join("rofi-websearch-data.json"))?.read_to_string(&mut s)?;
    let data: Vec<Search> = from_str(&s)?;

    let dt: HashMap<_, _> = data.into_iter().map(|d| (d.id.clone(), d)).collect();

    Ok(dt)
}

#[derive(Deserialize, Debug)]
pub struct Search {
    #[serde(rename = "s")]
    pub title: String,
    #[serde(rename = "d")]
    pub default_url: String,
    #[serde(rename = "t")]
    pub id: String,
    #[serde(rename = "u")]
    pub url: String,
}
