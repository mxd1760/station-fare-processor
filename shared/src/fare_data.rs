use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedFareData {
    pub kiosk_to_lu: HashMap<String, String>,
    pub lulu_to_fare: HashMap<(String, String), i64>,
}
