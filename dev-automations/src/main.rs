use clap::{Arg, Command, arg, command};
use shared::fare_data::GeneratedFareData;
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use crate::{
    map::{StationMap, load_map},
    mqtt::test_mqtt,
    rules::{DistanceItem, FareProcessingRules, load_fare_rules},
};

mod map;
mod mqtt;
mod rules;
mod simulator;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("test-mqtt").about("starts mqtt client loop and publishes 2 commands"),
        )
        .subcommand(
            Command::new("solve-fares")
                .about(
                    "read in map data with given ruleset to generate fare file for use on server",
                )
                .arg(
                    arg!(--map <map>)
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true)
                        .help("Path to map file"),
                )
                .arg(
                    arg!(--rules <rules>)
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true)
                        .help("Path to fare rules file"),
                )
                .arg(
                    arg!(--output <output>)
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(false)
                        .help("Path to output file"),
                ),
        )
        .subcommand(
            Command::new("simulate")
                .about("starts simulator with given map")
                .arg(
                    Arg::new("path")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("test-mqtt", _sub_matches)) => Ok(test_mqtt()),
        Some(("solve-fares", sub_matches)) => {
            let map_path = sub_matches.get_one::<PathBuf>("map").unwrap();
            let rules_path = sub_matches.get_one::<PathBuf>("rules").unwrap();
            let output_path = match sub_matches.get_one::<PathBuf>("output") {
                Some(v) => v,
                None => &PathBuf::from("./data/generated/output.fares"),
            };
            solve_fares(map_path, rules_path, output_path)
        }
        Some(("simulate", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("path");

            Ok(simulator::main(path)?)
        }
        _ => todo!(),
    }
}

fn solve_fares(
    map_path: &Path,
    rules_path: &Path,
    output_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let map = load_map(map_path)?;
    let rules = load_fare_rules(rules_path)?;
    let fares: GeneratedFareData;
    match rules {
        FareProcessingRules::Distance(max_fare, distance_fares_table) => {
            println!("parsing based on distance rules");
            fares = floyd_warshall(&map, max_fare, &distance_fares_table);
        }
        _ => {
            println!("oops, you provided an invalid fare processing mode");
            fares = GeneratedFareData {
                kiosk_to_lu: HashMap::new(),
                lulu_to_fare: HashMap::new(),
            };
        }
    }
    println!("{fares:?}");
    save_fares(output_file, fares)?;
    Ok(())
}

fn calculate_fare(distance: f64, max_fare: i64, distance_fares_table: &[DistanceItem]) -> i64 {
    for item in distance_fares_table {
        if distance <= item.max_distance {
            return item.fare;
        }
    }

    max_fare
}

fn floyd_warshall(
    map: &StationMap,
    max_fare: i64,
    distance_fares_table: &[DistanceItem],
) -> GeneratedFareData {
    let station_ids: Vec<String> = map.stations.keys().cloned().collect();
    let n = station_ids.len();
    let mut distances = vec![vec![f64::INFINITY; n]; n];

    // set identity
    for i in 0..n {
        distances[i][i] = 0.0;
    }

    let index: HashMap<&String, usize> = station_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (id, i))
        .collect();

    // set unit fares as defined in map
    for connection in &map.connections {
        let from = index[&connection.from];
        let to = index[&connection.to];

        if let Some(distance) = connection.distance {
            distances[from][to] = distance;
            distances[to][from] = distance;
        }
    }

    // calculate routes that take more than 1 unit of travel
    for k in 0..n {
        for source in 0..n {
            for dest in 0..n {
                let through_k = distances[source][k] + distances[k][dest];

                if through_k < distances[source][dest] {
                    distances[source][dest] = through_k;
                }
            }
        }
    }

    let mut kiosk_to_lu = HashMap::new();
    let mut lulu_to_fare = HashMap::new();

    // logical units should be stations
    for (station_id, station) in &map.stations {
        for kiosk in &station.kiosks {
            kiosk_to_lu.insert(kiosk.clone(), station_id.clone());
        }
    }

    // calculate given fare for each kind of trip
    for i in 0..n {
        for j in 0..n {
            let distance = distances[i][j];

            if distance.is_finite() {
                let fare = calculate_fare(distance, max_fare, distance_fares_table);

                lulu_to_fare.insert((station_ids[i].clone(), station_ids[j].clone()), fare);
            }
        }
    }

    GeneratedFareData {
        kiosk_to_lu,
        lulu_to_fare,
    }
}

// fn dijkstra(
//     _map: StationMap,
//     _max_fare: i64,
//     _distance_fares_table: Vec<DistanceItem>,
// ) -> GeneratedFareData {
//     todo!();
// }

fn save_fares(path: &Path, fares: GeneratedFareData) -> Result<(), Box<dyn Error>> {
    let encoded = postcard::to_stdvec(&fares)?;
    std::fs::write(path, encoded)?;
    Ok(())
}
