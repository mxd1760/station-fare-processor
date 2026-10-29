use std::path::PathBuf;

use crate::{map::load_map, simulator::simulation_app::SimulationApp};

mod actors;
mod drawable_elements;
mod simulation_app;
mod simulator_util;

pub fn main(path_arg: Option<&PathBuf>) -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let path = if let Some(p) = path_arg {
        p
    } else {
        &PathBuf::from("data/maps/simple_test_map.toml".to_owned())
    };
    let station_map = load_map(path).unwrap();

    eframe::run_native(
        "Station Fare Processor Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(SimulationApp::from_station_map(&station_map)))),
    )
}
