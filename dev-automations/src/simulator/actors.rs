use eframe::egui::{self, Color32, Painter};
use rand::{random_range, seq::IndexedRandom};

use crate::simulator::{
    drawable_elements::{DrawingPath, Element, Point, StationId},
    simulator_util::draw_rotated_rect,
};

type TrainId = i64;
type PassengerId = i64;

const TRAIN_TICKS_AT_STATION:i32 = 30;
const TRAIN_SIZE_AT_STATION:f32 = 8.0;
const TRAIN_SIZE_X: f32 = 30.0;
const TRAIN_SIZE_Y: f32 = 15.0;
const TRAIN_SPEED: f32 = 0.01;

pub enum Actor {
    Train {
        id: TrainId,
        color: Option<Color32>,
        position: TrainPos,
        route: Vec<String>,
        route_index: usize,
        passengers: Vec<PassengerId>,
        max_capacity: usize,
        wait_counter:i32,
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
                route: _,
                route_index: _,
                passengers: _num_passengers,
                max_capacity: _,
                color: _, wait_counter } => {
                match position {
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
                    TrainPos::AtStation(s, _) => {
                        if *wait_counter>0{
                            *wait_counter -= 1;
                        }else{
                            let options:Vec<&DrawingPath> = static_elements.iter().filter_map(|v|{
                                match v{
                                    Element::Path(drawing_path) => {
                                        match drawing_path{
                                            DrawingPath::Line { start, end, width: _, color: _ } => if start.0==*s || end.0==*s {Some(drawing_path)} else {None},
                                        }
                                    },
                                    Element::Circle { center: _, radius: _, color: _, name: _ } => None,
                                    Element::Text { position: _, text: _, color: _ } => None,
                                }
                            }).collect();
                            let connection = *options.choose(&mut rand::rng()).unwrap();
                            let going_forward = match connection{
                                DrawingPath::Line { start, end: _, width: _, color: _ } => start.0 == *s,
                            };
                            *position = TrainPos::Enroute{ connection:connection.clone(), interpolation: if going_forward {0.0} else{1.0}, going_forward };
                        }
                    }
                }
            }
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
                position,
                id: _,
                route: _,
                route_index: _,
                passengers: _,
                max_capacity: _,
                color, wait_counter } => match position {
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
    pub fn train_from(element: &Element) -> Option<Self> {
        match element {
            Element::Path(drawing_path) => Some(Actor::Train {
                id: 1,
                position: TrainPos::Enroute {
                    connection: drawing_path.clone(),
                    interpolation: 0.5,
                    going_forward: true,
                },
                route: vec![],
                route_index: 0,
                passengers: vec![],
                max_capacity: 10,
                color: None,
                wait_counter:0,
            }),
            Element::Circle {
                center: _,
                radius: _,
                color: _,
                name: _,
            } => todo!(),
            Element::Text {
                position: _,
                text: _,
                color: _,
            } => None,
        }
    }
}
