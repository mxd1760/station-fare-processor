use std::{collections::HashMap, time::Duration};

use eframe::egui;

use crate::{
    map::StationMap,
    simulator::{
        actors::Actor,
        drawable_elements::{DrawingPath, Element, Point},
        simulator_util::solve_station_positioning,
    },
};

#[derive(Default)]
pub struct SimulationApp {
    static_elements: Vec<Element>,
    actors: Vec<Actor>,
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
        }
    }

    pub fn from_station_map(map: &StationMap) -> Self {
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
        let train = Actor::train_from(elements.last().unwrap()).unwrap();

        Self {
            static_elements: elements,
            actors: vec![train],
        }
    }
}

impl eframe::App for SimulationApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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
        });
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for actor in &mut self.actors {
            actor.update(&self.static_elements);
        }
        ctx.request_repaint_after(Duration::from_millis(50))
    }
}
