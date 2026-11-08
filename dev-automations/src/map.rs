use serde::Deserialize;
use std::{collections::HashMap, error::Error, path::Path};

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
    pub routes: HashMap<String, Route>,
    pub train_cfg_schema: HashMap<String, TrainCfgSchema>,
}

#[derive(Debug, Deserialize)]
pub struct TrainCfgSchema {
    pub trains: Vec<TrainConfiguration>,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub name: String,
    pub mode: crate::simulator::actors::RouteMode,
    pub stations: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrainConfiguration {
    pub name: String,
    pub color: String,
    pub route_id: String,
    pub starting_index: usize,
}

pub fn load_map(path: &Path) -> Result<StationMap, Box<dyn Error>> {
    let contents = std::fs::read_to_string(path)?;
    let map = toml::from_str(&contents)?;
    Ok(map)
}
