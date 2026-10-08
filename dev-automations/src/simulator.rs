use std::{collections::HashMap, path::PathBuf};

use eframe::egui::{self, Painter};

use crate::map::{StationMap, load_map};

pub fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let path = PathBuf::from("data/maps/simple_test_map.toml".to_owned());
    let station_map = load_map(&path).unwrap();

    eframe::run_native(
        "Station Fare Processor Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(SimulationApp::from_station_map(&station_map)))),
    )
}





#[derive(Default)]
struct SimulationApp{
    visualization: Vec<Element>
}

impl SimulationApp{
    fn mock()->Self{
        let a = Point { x: 100.0, y: 150.0 };
        let b = Point { x: 300.0, y: 150.0 };
        let c = Point { x: 400.0, y: 300.0 };
        let d = Point { x: 500.0, y: 150.0 };
        let text_offset = Point{ x: 0.0,y:50.0};


        Self { visualization: vec![
            Element::Path(DrawingPath::Line { start: a.clone(), end: b.clone(), width: 5.0, color: Some(egui::Color32::RED) }),// 1-2 line
            Element::Path(DrawingPath::Line { start: b.clone(), end: c.clone(), width: 5.0, color: Some(egui::Color32::GREEN) }),// 2-3 line
            Element::Path(DrawingPath::Line { start: b.clone(), end: d.clone(), width: 5.0, color: Some(egui::Color32::BLUE)}),// 2-4 line
            Element::Path(DrawingPath::Line { start: d.clone(), end: c.clone(), width: 5.0, color: Some(egui::Color32::PURPLE) }),// 3-4 line
            Element::Circle { center: a.clone(), radius: 12.0 , color: None },// station 1 circle
            Element::Circle { center: b.clone(), radius: 12.0 , color: None },// station 2 circle
            Element::Circle { center: c.clone(), radius: 12.0 , color: None },// station 3 circle
            Element::Circle { center: d.clone(), radius: 12.0 , color: None },// station 4 circle
            Element::Text { position: a.clone().plus(&text_offset), text: "Station 1".to_owned(), color: None },// station 1 text
            Element::Text { position: b.clone().plus(&text_offset), text: "Station 2".to_owned(), color: None },// station 2 text
            Element::Text { position: c.clone().plus(&text_offset), text: "Station 3".to_owned(), color: None },// station 3 text
            Element::Text { position: d.clone().plus(&text_offset), text: "Station 4".to_owned(), color: None },// station 4 text
        ] }
    }

    fn from_station_map(map:&StationMap) -> Self{
        let mut elements = Vec::new();
        let mut positions = HashMap::new();

        for (index, name) in map.stations.keys().enumerate() {
            let position = Point {
                x: index as f32 * 100.0 + 100.0,
                y: 200.0 + (index as f32 * 1.7).sin() * 100.0,
            };
            let text_offset = Point{ x: 0.0,y:50.0};
            positions.insert(name, position);
                
            elements.push(Element::Circle {
                center: position,
                radius: 10.0,
                color: None,
            });

            elements.push(Element::Text {
                position: position.clone().plus(&text_offset),
                text: name.clone(),
                color: None,
            });
        }

        for connection in &map.connections {
            if let (Some(&start), Some(&end)) = (
                positions.get(&connection.from),
                positions.get(&connection.to),
            ) {
                elements.push(Element::Path(DrawingPath::Line {
                    start,
                    end,
                    width: 2.0,
                    color: None,
                }));
            }
        }

        Self{ visualization:elements }

    }
}


enum DrawingPath{
    Line {
        start: Point,
        end: Point,
        width: f32,
        color:Option<egui::Color32>
    },
    //Bezier
    //PolyLine
    //Complex Curve
}

enum Element {
    Path(DrawingPath),
    Circle {
        center: Point,
        radius: f32,
        color:Option<egui::Color32>
    },
    Text {
        position: Point,
        text: String,
        color:Option<egui::Color32>
    },
}

impl Element{
    fn draw(&self,painter:&Painter,rect:&egui::Rect,font:&egui::FontId){
        match self{
            Element::Path(p)=>{
                match p{
                    DrawingPath::Line { start, end, width, color } => {
                        let track = egui::Stroke::new(*width, match color { Some(v)=>*v,None=>egui::Color32::LIGHT_GRAY});
                        painter.line_segment([start.to_pos2(*rect), end.to_pos2(*rect)], track);
                    }
                }
            },
            Element::Circle { center, radius, color } => {
                let real_color = match color {
                    Some(c) => c,
                    None => &egui::Color32::LIGHT_GRAY,
                };
                painter.circle_filled(
                    center.to_pos2(*rect),
                    *radius,
                    *real_color,
                );
            },
            Element::Text { position, text, color } => {
                painter.text(
                    position.to_pos2(*rect),
                    egui::Align2::CENTER_CENTER,
                    text.as_str(),
                    font.clone(),
                    match color{Some(v)=>*v,None=>egui::Color32::WHITE},
                );
            },
        }
    }
}

#[derive(Clone, Copy)]
struct Point{
    x:f32,
    y:f32
}

impl Point{
    fn plus(&mut self,other:&Self)->Self{
        self.x += other.x;
        self.y += other.y;
        *self
    }
    fn to_pos2(&self,rect:egui::Rect)->egui::Pos2{
        rect.left_top()+egui::vec2(self.x,self.y)
    }
}

impl eframe::App for SimulationApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Station Fare Processor");

            ui.separator();

            let available_size = ui.available_size();

            let (response, painter) =
                ui.allocate_painter(available_size, egui::Sense::hover());

            let rect = response.rect;
            let font = egui::FontId::proportional(16.0);

            for element in &self.visualization{
                element.draw(&painter,&rect,&font);
            }

        });
    }
}