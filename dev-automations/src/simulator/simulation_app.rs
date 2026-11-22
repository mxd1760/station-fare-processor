use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use eframe::egui;

use crate::{
    map::{Route, StationMap, TrainConfiguration},
    simulator::{
        actors::{Actor, PassengerPos, TrainPos},
        drawable_elements::{DrawingPath, Element, Point},
        simulator_util::{draw_passenger_count, random_id, solve_station_positioning},
    },
};

const PASSENGER_SPAWN_DELAY: u64 = 2;
const PASSENGER_DISPLAY_OFFSET: Point = Point { x: 10.0, y: 10.0 };
const MAX_SIMULATED_PASSENGERS: usize = 100;

pub struct SimulationApp {
    static_elements: Vec<Element>,
    actors: Vec<Actor>,
    show_info_panel: bool,
    train_routes: HashMap<String, Route>,
    passenger_spawn_timer: Instant,
}

impl SimulationApp {
    fn _mock() -> Self {
        let a = Point { x: 100.0, y: 150.0 };
        let b = Point { x: 300.0, y: 150.0 };
        let c = Point { x: 400.0, y: 300.0 };
        let d = Point { x: 500.0, y: 150.0 };
        let text_offset = Point { x: 0.0, y: 50.0 };

        Self {
            static_elements: vec![
                Element::Path(DrawingPath::Line {
                    start: ("Station 1".into(), a.clone()),
                    end: ("Station 2".into(), b.clone()),
                    width: 5.0,
                    color: Some(egui::Color32::RED),
                }), // 1-2 line
                Element::Path(DrawingPath::Line {
                    start: ("Station 2".into(), b.clone()),
                    end: ("Station 3".into(), c.clone()),
                    width: 5.0,
                    color: Some(egui::Color32::GREEN),
                }), // 2-3 line
                Element::Path(DrawingPath::Line {
                    start: ("Station 2".into(), b.clone()),
                    end: ("Station 4".into(), d.clone()),
                    width: 5.0,
                    color: Some(egui::Color32::BLUE),
                }), // 2-4 line
                Element::Path(DrawingPath::Line {
                    start: ("Station 4".into(), d.clone()),
                    end: ("Station 3".into(), c.clone()),
                    width: 5.0,
                    color: Some(egui::Color32::PURPLE),
                }), // 3-4 line
                Element::Circle {
                    center: a.clone(),
                    radius: 12.0,
                    color: None,
                    name: "Station 1".into(),
                }, // station 1 circle
                Element::Circle {
                    center: b.clone(),
                    radius: 12.0,
                    color: None,
                    name: "Station 2".into(),
                }, // station 2 circle
                Element::Circle {
                    center: c.clone(),
                    radius: 12.0,
                    color: None,
                    name: "Station 3".into(),
                }, // station 3 circle
                Element::Circle {
                    center: d.clone(),
                    radius: 12.0,
                    color: None,
                    name: "Station 4".into(),
                }, // station 4 circle
                Element::Text {
                    position: a.clone().plus(&text_offset),
                    text: "Station 1".to_owned(),
                    color: None,
                }, // station 1 text
                Element::Text {
                    position: b.clone().plus(&text_offset),
                    text: "Station 2".to_owned(),
                    color: None,
                }, // station 2 text
                Element::Text {
                    position: c.clone().plus(&text_offset),
                    text: "Station 3".to_owned(),
                    color: None,
                }, // station 3 text
                Element::Text {
                    position: d.clone().plus(&text_offset),
                    text: "Station 4".to_owned(),
                    color: None,
                }, // station 4 text
            ],
            actors: vec![],
            show_info_panel: true,
            train_routes: HashMap::new(),
            passenger_spawn_timer: Instant::now(),
        }
    }

    pub fn from_station_map(map: &StationMap, cfg_schema: Option<&String>) -> Self {
        let trains: &Vec<TrainConfiguration> = match cfg_schema {
            Some(name) => &map.train_cfg_schema.get(name).unwrap().trains,
            None => &map.train_cfg_schema.get("one").unwrap().trains,
        };
        let mut elements = Vec::new();
        let positions: HashMap<String, Point> = solve_station_positioning(map);

        for (name, position) in &positions {
            elements.push(Element::Circle {
                center: *position,
                radius: 10.0,
                color: None,
                name: name.clone(),
            });

            let text_offset = Point { x: 0.0, y: 50.0 };
            elements.push(Element::Text {
                position: position.clone().plus(&text_offset),
                text: name.clone(),
                color: None,
            });
        }

        for connection in &map.connections {
            if let (Some(&start_pos), Some(&end_pos)) = (
                positions.get(&connection.from),
                positions.get(&connection.to),
            ) {
                let start = (connection.from.clone(), start_pos);
                let end = (connection.to.clone(), end_pos);
                elements.push(Element::Path(DrawingPath::Line {
                    start,
                    end,
                    width: 2.0,
                    color: None,
                }));
            }
        }

        let mut actors = vec![];
        for train in trains {
            let route = map.routes.get(&train.route_id).unwrap();
            let mut t = Actor::train_from_config_and_route(&train, &route).unwrap();
            t.valid_route_check(&elements);
            actors.push(t);
        }

        Self {
            static_elements: elements,
            actors,
            show_info_panel: true,
            train_routes: map.routes.clone(),
            passenger_spawn_timer: Instant::now(),
        }
    }
}

impl Default for SimulationApp {
    fn default() -> Self {
        Self {
            static_elements: Default::default(),
            actors: Default::default(),
            show_info_panel: Default::default(),
            train_routes: Default::default(),
            passenger_spawn_timer: Instant::now(),
        }
    }
}

impl eframe::App for SimulationApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.show_info_panel {
            egui::Panel::right("info").show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::CollapsingHeader::new(egui::RichText::new("Trains").size(18.0).strong())
                        .default_open(true)
                        .show(ui, |ui| {
                            self.actors
                                .iter()
                                .filter(|v| matches!(v, Actor::Train { .. }))
                                .for_each(|train| {
                                    let (id, color) = match train {
                                        Actor::Train { id, color, .. } => (id, color),
                                        Actor::Passenger { .. } => unreachable!(),
                                    };
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(&train.info_title())
                                                .size(14.0)
                                                .strong(),
                                        );

                                        let (rect, _) = ui.allocate_exact_size(
                                            egui::vec2(12.0, 12.0),
                                            egui::Sense::hover(),
                                        );
                                        ui.painter().circle_filled(
                                            rect.center(),
                                            6.0,
                                            color.unwrap(),
                                        );
                                    });
                                    ui.indent(id, |ui| {
                                        ui.label(&train.info_subtitle());
                                    });
                                });
                        });
                    egui::CollapsingHeader::new(
                        egui::RichText::new("Passengers").size(18.0).strong(),
                    )
                    .default_open(true)
                    .show(ui, |ui| {
                        self.actors
                            .iter()
                            .filter(|v| matches!(v, Actor::Passenger { .. }))
                            .for_each(|passenger| {
                                // ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&passenger.info_title())
                                        .size(14.0)
                                        .strong(),
                                );
                                // });
                                ui.indent(passenger.info_title(), |ui| {
                                    ui.label(&passenger.info_subtitle());
                                });
                            })
                    })
                });
            });
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Station Fare Processor");

            ui.separator();

            let available_size = ui.available_size();

            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

            let rect = response.rect;
            let font = egui::FontId::proportional(16.0);

            for element in &self.static_elements {
                element.draw(&painter, &rect, &font);
            }
            for actor in &self.actors {
                actor.draw(&painter, &rect);
            }

            let mut passengers_on_trains = HashMap::new();
            let mut passengers_at_stations = HashMap::new();

            self.actors.iter().for_each(|actor| match actor {
                Actor::Train { .. } => (),
                Actor::Passenger { position, .. } => match position {
                    super::actors::PassengerPos::OnTrain(id) => {
                        *passengers_on_trains.entry(id).or_insert(0) += 1;
                    }
                    super::actors::PassengerPos::AtStation(id, point) => {
                        passengers_at_stations.entry(id).or_insert((0, point)).0 += 1;
                    }
                },
            });

            for (train_id, count) in passengers_on_trains {
                let train_pos = self
                    .actors
                    .iter()
                    .find(|actor| match actor {
                        Actor::Train { id, .. } => id == train_id,
                        Actor::Passenger { .. } => false,
                    })
                    .map(|train| match train {
                        Actor::Train { position, .. } => position,
                        Actor::Passenger { .. } => unreachable!(),
                    })
                    .unwrap();
                match train_pos {
                    TrainPos::Enroute {
                        connection,
                        interpolation,
                        ..
                    } => match connection {
                        DrawingPath::Line { start, end, .. } => {
                            let delta = end.clone().1.minus(&start.1);
                            let mut center = start.clone();
                            center.1.plus(&delta.clone().times(&interpolation));

                            center.1.plus(&PASSENGER_DISPLAY_OFFSET);
                            draw_passenger_count(&painter, center.1.to_pos2(rect), count);
                        }
                    },
                    TrainPos::AtStation(id, point) => {
                        passengers_at_stations.entry(id).or_insert((0, point)).0 += 1;
                    }
                }
            }
            for station_data in passengers_at_stations.values() {
                let mut b = station_data.1.clone();
                b.plus(&PASSENGER_DISPLAY_OFFSET);
                draw_passenger_count(&painter, b.to_pos2(rect), station_data.0);
            }
        });
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut actor_requests = vec![];
        for actor in &mut self.actors {
            actor_requests.push(actor.update(&self.static_elements));
        }

        let mut station_point_map = HashMap::new();
        for elem in &self.static_elements {
            match elem {
                Element::Path(_) => (),
                Element::Circle { center, name, .. } => {
                    station_point_map.insert(name.clone(), center.clone());
                    ()
                }
                Element::Text { .. } => (),
            }
        }
        if self.passenger_spawn_timer.elapsed() >= Duration::from_secs(PASSENGER_SPAWN_DELAY) {
            if self.actors.len() < MAX_SIMULATED_PASSENGERS {
                self.actors.push(Actor::new_passenger(
                    random_id(5),
                    random_id(5),
                    &self.train_routes,
                    &station_point_map,
                ));
                self.passenger_spawn_timer = Instant::now();
            }
        }

        for request in actor_requests {
            // println!("{:?}",request);
            match request {
                super::actors::ActorRequest::None => (),
                super::actors::ActorRequest::DumpPassengersAtStation(
                    train_id,
                    station_id,
                    point,
                ) => {
                    let passenger_ids = self
                        .actors
                        .iter_mut()
                        .find(|actor| match actor {
                            Actor::Train { id, .. } => *id == train_id,
                            Actor::Passenger { .. } => false,
                        })
                        .map(|actor| match actor {
                            Actor::Train { passengers, .. } => {
                                let out = passengers.clone();
                                *passengers = vec![];
                                out
                            }
                            Actor::Passenger { .. } => unreachable!(),
                        })
                        .unwrap();
                    for actor in self.actors.iter_mut() {
                        match actor {
                            Actor::Train { .. } => continue,
                            Actor::Passenger { id, position, .. } => {
                                if passenger_ids.contains(id) {
                                    *position = PassengerPos::AtStation(station_id.clone(), point);
                                }
                            }
                        }
                    }
                }
                super::actors::ActorRequest::PassengerRequestToBoardTrainFromTo(
                    passenger_id,
                    id_from,
                    id_to,
                ) => {
                    let trains_at_from = self.actors.iter_mut().filter(|actor| match actor {
                        Actor::Train { position, .. } => match position {
                            TrainPos::Enroute { .. } => false,
                            TrainPos::AtStation(sid, _) => *sid == id_from,
                        },
                        Actor::Passenger { .. } => false,
                    });
                    let mut assignment = None;
                    for train in trains_at_from {
                        match train {
                            Actor::Train {
                                id,
                                route,
                                route_index,
                                passengers,
                                max_capacity,
                                ..
                            } => {
                                if route.get(*route_index + 1) == Some(&id_to)
                                    && passengers.len() < *max_capacity
                                {
                                    passengers.push(passenger_id.clone());
                                    assignment = Some((id.clone(), passenger_id));
                                    break;
                                }
                            }
                            Actor::Passenger { .. } => unreachable!(),
                        }
                    }
                    if let Some((tid, pid)) = assignment {
                        let p = self
                            .actors
                            .iter_mut()
                            .find(|actor| match actor {
                                Actor::Train { .. } => false,
                                Actor::Passenger { id, .. } => *id == pid,
                            })
                            .unwrap();
                        match p {
                            Actor::Train { .. } => unreachable!(),
                            Actor::Passenger { position, .. } => {
                                *position = PassengerPos::OnTrain(tid)
                            }
                        }
                    }
                }
                super::actors::ActorRequest::DeleteTrain(train_id) => {
                    let index = self
                        .actors
                        .iter()
                        .enumerate()
                        .find(|(_, actor)| match actor {
                            Actor::Train { id, .. } => *id == train_id,
                            Actor::Passenger { .. } => false,
                        })
                        .map(|(index, _)| index)
                        .unwrap();
                    self.actors.remove(index);
                }
                super::actors::ActorRequest::DeletePassenger(passenger_id) => {
                    let index = self
                        .actors
                        .iter()
                        .enumerate()
                        .find(|(_, actor)| match actor {
                            Actor::Train { .. } => false,
                            Actor::Passenger { id, .. } => *id == passenger_id,
                        })
                        .map(|(index, _)| index)
                        .unwrap();
                    self.actors.remove(index);
                }
            }
        }

        ctx.request_repaint_after(Duration::from_millis(50))
    }
}
