use chrono::prelude::*;
use eframe::egui::{self, Color32, Painter};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

use crate::{
    map::{Route, TrainConfiguration},
    simulator::{
        drawable_elements::{DrawingPath, Element, Point, StationId},
        simulator_util::{draw_rotated_rect, find_route_dijkstra},
    },
};

type TrainId = String;
type PassengerId = String;

const TRAIN_TICKS_AT_STATION: i32 = 30;
const TRAIN_SIZE_AT_STATION: f32 = 8.0;
const TRAIN_SIZE_X: f32 = 30.0;
const TRAIN_SIZE_Y: f32 = 15.0;
const TRAIN_SPEED: f32 = 0.01;
const TRAIN_PASSANGER_CAP: usize = 10;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RouteMode {
    #[serde(rename = "random")]
    RandomConnection,
    #[serde(rename = "random-route")]
    RandomRoute,
    #[serde(rename = "linear")]
    LinearRoute,
    #[serde(rename = "loop")]
    LoopRoute,
}

pub enum Actor {
    Train {
        id: TrainId,
        color: Option<Color32>,
        position: TrainPos,
        route: Vec<String>,
        route_index: usize,
        passengers: Vec<PassengerId>,
        max_capacity: usize,
        wait_counter: i32,
        route_mode: RouteMode,
        route_name: String,
        connection_name: String,
    },
    Passenger {
        id: PassengerId,
        card_id: String,
        position: PassengerPos,
        destination_station: StationId,
        route: Vec<StationId>,
        route_index: usize,
    },
}
pub enum TrainPos {
    Enroute {
        connection: DrawingPath,
        interpolation: f32,
        going_forward: bool,
    },
    AtStation(StationId, Point),
}
pub enum PassengerPos {
    OnTrain(TrainId),
    AtStation(StationId, Point),
    // Roaming(Point),
}

impl Actor {
    pub fn update(&mut self, static_elements: &Vec<Element>) {
        match self {
            Actor::Train {
                id: _,
                position,
                route,
                route_index,
                passengers: _num_passengers,
                max_capacity: _,
                color: _,
                wait_counter,
                route_mode,
                route_name,
                connection_name,
            } => match position {
                TrainPos::Enroute {
                    connection,
                    interpolation,
                    going_forward,
                } => {
                    *interpolation = if *going_forward {
                        *interpolation + TRAIN_SPEED
                    } else {
                        *interpolation - TRAIN_SPEED
                    };
                    if *interpolation > 1.0 {
                        let new_position = match connection {
                            DrawingPath::Line {
                                start: _,
                                end,
                                width: _,
                                color: _,
                            } => {
                                let station: &Point = static_elements
                                    .iter()
                                    .filter_map(|v| match v {
                                        Element::Path(_drawing_path) => None,
                                        Element::Circle {
                                            center,
                                            radius: _,
                                            color: _,
                                            name,
                                        } => {
                                            if *name == end.0 {
                                                Some(center)
                                            } else {
                                                None
                                            }
                                        }
                                        Element::Text {
                                            position: _,
                                            text: _,
                                            color: _,
                                        } => None,
                                    })
                                    .collect::<Vec<&Point>>()
                                    .get(0)
                                    .unwrap();
                                TrainPos::AtStation(end.0.clone(), station.clone())
                            }
                        };

                        *position = new_position;
                        *wait_counter = TRAIN_TICKS_AT_STATION;
                    } else if *interpolation < 0.0 {
                        let new_position = match connection {
                            DrawingPath::Line {
                                start,
                                end: _,
                                width: _,
                                color: _,
                            } => {
                                let station: &Point = static_elements
                                    .iter()
                                    .filter_map(|v| match v {
                                        Element::Path(_drawing_path) => None,
                                        Element::Circle {
                                            center,
                                            radius: _,
                                            color: _,
                                            name,
                                        } => {
                                            if *name == start.0 {
                                                Some(center)
                                            } else {
                                                None
                                            }
                                        }
                                        Element::Text {
                                            position: _,
                                            text: _,
                                            color: _,
                                        } => None,
                                    })
                                    .collect::<Vec<&Point>>()
                                    .get(0)
                                    .unwrap();
                                TrainPos::AtStation(start.0.clone(), station.clone())
                            }
                        };

                        *position = new_position;
                        *wait_counter = TRAIN_TICKS_AT_STATION;
                    }
                }
                TrainPos::AtStation(here, _) => {
                    if *wait_counter > 0 {
                        *wait_counter -= 1;
                    } else {
                        *position = match *route_mode {
                            RouteMode::RandomConnection => {
                                Self::next_train_path_by_random_choice(here, static_elements)
                            }
                            RouteMode::RandomRoute => {
                                if route.len() == 0 || *route_index >= route.len() - 1 {
                                    let mut to: StationId = here.clone();
                                    while to == *here {
                                        to = static_elements
                                            .iter()
                                            .filter_map(|v| match v {
                                                Element::Path(_) => None,
                                                Element::Circle { name, .. } => Some(name.clone()),
                                                Element::Text { .. } => None,
                                            })
                                            .collect::<Vec<StationId>>()
                                            .choose(&mut rand::rng())
                                            .unwrap()
                                            .to_string();
                                    }
                                    *route_name = format!("to {}", to);
                                    *route =
                                        find_route_dijkstra(static_elements, here, &to).unwrap();
                                    *route_index = 0;
                                }
                                *route_index += 1;
                                let out = Self::next_train_path_by_route(
                                    here,
                                    route.get(*route_index).unwrap(),
                                    static_elements,
                                );
                                *connection_name = out.1;
                                out.0
                            }
                            RouteMode::LinearRoute => {
                                if *route_index >= route.len() - 1 {
                                    *route_index = 0;
                                    route.reverse();
                                }
                                *route_index += 1;
                                let out = Self::next_train_path_by_route(
                                    here,
                                    route.get(*route_index).unwrap(),
                                    static_elements,
                                );
                                *connection_name = out.1;
                                out.0
                            }
                            RouteMode::LoopRoute => {
                                if *route_index >= route.len() - 1 {
                                    *route_index = 0;
                                }
                                *route_index += 1;
                                let out = Self::next_train_path_by_route(
                                    here,
                                    route.get(*route_index).unwrap(),
                                    static_elements,
                                );
                                *connection_name = out.1;
                                out.0
                            }
                        }
                    }
                }
            },
            Actor::Passenger {
                id: _,
                card_id: _,
                position: _,
                destination_station: _,
                route: _,
                route_index: _,
            } => todo!(),
        }
    }
    pub fn draw(&self, painter: &Painter, rect: &egui::Rect) {
        match self {
            Actor::Train {
                position, color, ..
            } => match position {
                TrainPos::Enroute {
                    connection,
                    interpolation,
                    going_forward: _,
                } => match connection {
                    DrawingPath::Line {
                        start,
                        end,
                        width: _,
                        color: _,
                    } => {
                        let delta = end.clone().1.minus(&start.1);
                        let mut center = start.clone();
                        center.1.plus(&delta.clone().times(&interpolation));
                        let size = egui::vec2(TRAIN_SIZE_X, TRAIN_SIZE_Y);
                        let angle = delta.y.atan2(delta.x);
                        let color = match color {
                            Some(c) => *c,
                            None => egui::Color32::BLUE,
                        };
                        draw_rotated_rect(painter, center.1.to_pos2(*rect), size, angle, color)
                    }
                },
                TrainPos::AtStation(_, p) => {
                    let color = match color {
                        Some(c) => *c,
                        None => egui::Color32::BLUE,
                    };
                    painter.circle_filled(p.to_pos2(*rect), TRAIN_SIZE_AT_STATION, color);
                }
            },
            Actor::Passenger {
                id: _,
                card_id: _,
                position: _,
                destination_station: _,
                route: _,
                route_index: _,
            } => {}
        }
    }
    pub fn _train_from_element(element: &Element) -> Option<Self> {
        match element {
            Element::Path(drawing_path) => Some(Actor::Train {
                id: Utc::now().to_string(),
                position: TrainPos::Enroute {
                    connection: drawing_path.clone(),
                    interpolation: 0.0,
                    going_forward: true,
                },
                route: vec![],
                route_index: 0,
                passengers: vec![],
                max_capacity: TRAIN_PASSANGER_CAP,
                color: None,
                wait_counter: 0,
                route_mode: RouteMode::RandomRoute,
                route_name: "Random".to_string(),
                connection_name: "Unknown".to_string(),
            }),
            Element::Circle {
                center,
                radius: _,
                color: _,
                name,
            } => Some(Actor::Train {
                id: Utc::now().to_string(),
                position: TrainPos::AtStation(name.clone(), center.clone()),
                route: vec![],
                route_index: 0,
                passengers: vec![],
                max_capacity: 10,
                color: None,
                wait_counter: 0,
                route_mode: RouteMode::RandomRoute,
                route_name: "Random".to_string(),
                connection_name: "Unknown".to_string(),
            }),
            Element::Text {
                position: _,
                text: _,
                color: _,
            } => None,
        }
    }
    pub fn train_from_config_and_route(cfg: &TrainConfiguration, route: &Route) -> Option<Self> {
        Some(Actor::Train {
            id: cfg.name.clone(),
            color: match cfg.color.as_str() {
                "BLACK" => Some(egui::Color32::BLACK),
                "DARK_GRAY" | "DARK_GREY" => Some(egui::Color32::DARK_GRAY),
                "GREY" | "GRAY" => Some(egui::Color32::GRAY),
                "LIGHT_GRAY" | "LIGHT_GREY" => Some(egui::Color32::LIGHT_GRAY),
                "WHITE" => Some(egui::Color32::WHITE),
                "BROWN" => Some(egui::Color32::BROWN),
                "DARK_RED" => Some(egui::Color32::DARK_RED),
                "RED" => Some(egui::Color32::RED),
                "LIGHT_RED" => Some(egui::Color32::LIGHT_RED),
                "CYAN" => Some(egui::Color32::CYAN),
                "MAGENTA" => Some(egui::Color32::MAGENTA),
                "YELLOW" => Some(egui::Color32::YELLOW),
                "ORANGE" => Some(egui::Color32::ORANGE),
                "LIGHT_YELLOW" => Some(egui::Color32::LIGHT_YELLOW),
                "KHAKI" => Some(egui::Color32::KHAKI),
                "DARK_GREEN" => Some(egui::Color32::DARK_GREEN),
                "GREEN" => Some(egui::Color32::GREEN),
                "LIGHT_GREEN" => Some(egui::Color32::LIGHT_GREEN),
                "DARK_BLUE" => Some(egui::Color32::DARK_BLUE),
                "BLUE" => Some(egui::Color32::BLUE),
                "LIGHT_BLUE" => Some(egui::Color32::LIGHT_BLUE),
                "PURPLE" => Some(egui::Color32::PURPLE),
                "GOLD" => Some(egui::Color32::GOLD),
                _ => todo!(), // try reading rgb before returning None
            },
            position: TrainPos::AtStation(
                route.stations.get(cfg.starting_index)?.clone(),
                Point { x: 100.0, y: 100.0 },
            ),
            route: route.stations.clone(),
            route_index: cfg.starting_index,
            passengers: vec![],
            max_capacity: TRAIN_PASSANGER_CAP,
            wait_counter: 0,
            route_mode: route.mode.clone(),
            route_name: route.name.clone(),
            connection_name: "Unknown".to_string(),
        })
    }

    fn next_train_path_by_random_choice(
        from: &StationId,
        static_elements: &Vec<Element>,
    ) -> TrainPos {
        let options: Vec<&DrawingPath> = static_elements
            .iter()
            .filter_map(|v| match v {
                Element::Path(drawing_path) => {
                    if drawing_path.station_a() == *from || drawing_path.station_b() == *from {
                        Some(drawing_path)
                    } else {
                        None
                    }
                }
                Element::Circle { .. } => None,
                Element::Text { .. } => None,
            })
            .collect();
        let connection = *options.choose(&mut rand::rng()).unwrap();
        let going_forward = connection.station_a() == *from;
        TrainPos::Enroute {
            connection: connection.clone(),
            interpolation: if going_forward { 0.0 } else { 1.0 },
            going_forward,
        }
    }
    fn next_train_path_by_route(
        from: &StationId,
        to: &StationId,
        static_elements: &[Element],
    ) -> (TrainPos,String) {
        
        let connection: DrawingPath = static_elements
            .iter()
            .filter_map(|v| match v {
                Element::Path(drawing_path) => Some(drawing_path),
                Element::Circle { .. } => None,
                Element::Text { .. } => None,
            })
            .find(|v| {
                (v.station_a() == *from && v.station_b() == *to)
                    || (v.station_b() == *from && v.station_a() == *to)
            })
            .unwrap()
            .clone();
        let going_forward = connection.station_a() == *from;
        (TrainPos::Enroute {
            connection,
            interpolation: if going_forward { 0.0 } else { 1.0 },
            going_forward,
        },format!("from {} to {}", from, to))
    }

    pub fn info_title(&self) -> String {
        match self {
            Actor::Train { id, route_name, .. } => format!("{} on {}", id, route_name),
            Actor::Passenger { .. } => todo!(),
        }
    }
    pub fn info_subtitle(&self) -> String {
        match self {
            Actor::Train {
                connection_name, ..
            } => format!("On Connection {}", connection_name),
            Actor::Passenger { .. } => todo!(),
        }
    }
}
