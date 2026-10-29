use std::{collections::HashMap, error::Error, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Station {
    #[serde(rename = "name")]
    pub _name: String,
    pub kiosks: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
    pub distance: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct StationMap {
    pub stations: HashMap<String, Station>,
    pub connections: Vec<Connection>,
}

pub fn load_map(path: &Path) -> Result<StationMap, Box<dyn Error>> {
    let contents = std::fs::read_to_string(path)?;
    let map = toml::from_str(&contents)?;
    Ok(map)
}
