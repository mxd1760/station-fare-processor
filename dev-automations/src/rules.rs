use std::{error::Error, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FareRulesConfig {
    pub mode: FareMode,
    pub max_fare: i64,
    #[serde(default)]
    pub brackets: Option<Vec<DistanceItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FareMode {
    Distance,
    Zone,
    Pair,
}

#[derive(Debug, Deserialize)]
pub struct DistanceItem {
    pub max_distance: f64,
    pub fare: i64,
}

#[derive(Debug)]
pub enum FareProcessingRules {
    Distance(i64, Vec<DistanceItem>),
    Segment(i64),
    Zone(i64),
    Pair(i64),
}

impl From<FareRulesConfig> for FareProcessingRules {
    fn from(config: FareRulesConfig) -> FareProcessingRules {
        match config.mode {
            FareMode::Distance => {
                FareProcessingRules::Distance(config.max_fare, config.brackets.unwrap())
            }
            FareMode::Zone => todo!(),
            FareMode::Pair => todo!(),
        }
    }
}

pub fn load_fare_rules(path: &Path) -> Result<FareProcessingRules, Box<dyn Error>> {
    let contents = std::fs::read_to_string(path)?;
    let raw_rules: FareRulesConfig = toml::from_str(&contents)?;
    let rules = FareProcessingRules::try_from(raw_rules)?;
    Ok(rules)
}
