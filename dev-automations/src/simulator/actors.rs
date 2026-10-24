use eframe::egui::{self, Painter};

use crate::simulator::{drawable_elements::DrawingPath, simulator_util::draw_rotated_rect};



type TrainId = i64;
type PassengerId = i64;
pub enum Actor{
    Train{
        id:TrainId,
        position:TrainPos,
        route:Vec<String>,
        route_index:usize,
        passengers:Vec<PassengerId>,
        max_capacity:usize,
    },
    Passenger{
        id:PassengerId,
        card_id:String,
        position:PassengerPos,
        destination_station:String,
        route:Vec<String>,
        route_index:usize
    },
}
pub enum TrainPos{
    Enroute{
        connection:DrawingPath,
        interpolation:f32,
        going_forward:bool
    },
    AtStation(String)
}
pub enum PassengerPos{
    OnTrain(TrainId),
    AtStation(String),
    // Roaming(Point),
}

const TRAIN_SIZE_X:f32 = 30.0;
const TRAIN_SIZE_Y:f32 = 15.0;
const TRAIN_SPEED:f32 = 0.01;

impl Actor{
    pub fn update(&mut self){
        match self{
            Actor::Train { id, position, route, route_index, passengers: num_passengers, max_capacity } => {
                match position{
                    TrainPos::Enroute { connection, interpolation, going_forward} => {
                        *interpolation = if *going_forward { *interpolation + TRAIN_SPEED } else {*interpolation-TRAIN_SPEED};
                        if *interpolation >1.0{
                            *interpolation = 1.0;
                            *going_forward = false;
                        }else if *interpolation<0.0{
                            *interpolation = 0.0;
                            *going_forward = true;
                        }
                    },
                    TrainPos::AtStation(_) => todo!(),
                }
            },
            Actor::Passenger { id, card_id, position, destination_station, route, route_index } => todo!(),
        }
    }
    pub fn draw(&self, painter:&Painter,rect:&egui::Rect){
        match self {
            Actor::Train { position, id:_, route: _, route_index: _, passengers: _, max_capacity: _ } => {
                match position{
                    TrainPos::Enroute { connection, interpolation, going_forward: _ } => {
                        match connection{
                            DrawingPath::Line { start, end, width:_, color:_ } => {
                                let delta = end.clone().minus(&start);
                                let mut center = start.clone();
                                center.plus(&delta.clone().times(&interpolation));
                                let size = egui::vec2(TRAIN_SIZE_X,TRAIN_SIZE_Y);
                                let angle = delta.y.atan2(delta.x);
                                draw_rotated_rect(
                                    painter,
                                    center.to_pos2(*rect),
                                    size,
                                    angle,
                                    egui::Color32::BLUE
                                )
                            },
                        }
                    },
                    TrainPos::AtStation(s) => {

                    },
                }
            },
            Actor::Passenger { id, card_id, position, destination_station, route, route_index } => {

            },
        }
    }
}

