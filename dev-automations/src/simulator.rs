use std::{collections::HashMap, path::PathBuf, time::Duration};

use eframe::{egui::{self, Painter}, wgpu::naga::MathFunction::Atan2};

use crate::map::{StationMap, load_map};

pub fn main(path_arg:Option<&PathBuf>) -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let path = if let Some(p) = path_arg{
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


type TrainId = i64;
type PassengerId = i64;
enum Actor{
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
enum TrainPos{
    Enroute{
        connection:DrawingPath,
        interpolation:f32
    },
    AtStation(String)
}
enum PassengerPos{
    OnTrain(TrainId),
    AtStation(String),
    // Roaming(Point),
}

const TRAIN_SIZE_X:f32 = 30.0;
const TRAIN_SIZE_Y:f32 = 15.0;

impl Actor{
    fn update(&mut self){
        match self{
            Actor::Train { id, position, route, route_index, passengers: num_passengers, max_capacity } => {

            },
            Actor::Passenger { id, card_id, position, destination_station, route, route_index } => todo!(),
        }
    }
    fn draw(&self, painter:&Painter,rect:&egui::Rect){
        match self {
            Actor::Train { position, id:_, route: _, route_index: _, passengers: _, max_capacity: _ } => {
                match position{
                    TrainPos::Enroute { connection, interpolation } => {
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


#[derive(Default)]
struct SimulationApp{
    static_elements: Vec<Element>,
    actors: Vec<Actor>
}

impl SimulationApp{
    fn mock()->Self{
        let a = Point { x: 100.0, y: 150.0 };
        let b = Point { x: 300.0, y: 150.0 };
        let c = Point { x: 400.0, y: 300.0 };
        let d = Point { x: 500.0, y: 150.0 };
        let text_offset = Point{ x: 0.0,y:50.0};


        Self { static_elements: vec![
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
        ],
            actors: vec![]}
    }

    fn from_station_map(map:&StationMap) -> Self{
        let mut elements = Vec::new();
        let positions:HashMap<String,Point> = solve_station_positioning(map);

        for (name,position) in &positions {
            elements.push(Element::Circle {
                center: *position,
                radius: 10.0,
                color: None,
            });
            
            let text_offset = Point{x:0.0, y:50.0};
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

        let mock_train_connection = DrawingPath::Line { start: Point{x:100.0,y:200.0}, end: Point{x:300.0,y:400.0}, width: 4.9, color: None };

        elements.push(Element::Path(mock_train_connection.clone()));

        Self{ 
            static_elements:elements, 
            actors: vec![
                Actor::Train { id: 1, position: TrainPos::Enroute { connection: mock_train_connection, interpolation: 0.5 }, route: vec![], route_index: 0, passengers: vec![], max_capacity: 10 
            }] }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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
    fn minus(&mut self,other:&Self)->Self{
        self.x -= other.x;
        self.y -= other.y;
        *self
    }
    fn to_pos2(&self,rect:egui::Rect)->egui::Pos2{
        rect.left_top()+egui::vec2(self.x,self.y)
    }
    fn times(&mut self, other:&f32)->Self{
        self.x *= other;
        self.y *= other;
        *self
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

            for element in &self.static_elements{
                element.draw(&painter,&rect,&font);
            }
            for actor in &self.actors{
                actor.draw(&painter,&rect);
            }
        });
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        for actor in &mut self.actors{
            actor.update();
        }
        ctx.request_repaint_after(
            Duration::from_millis(50)
        )
    }
}


// TODO: BENCHMARK ME
fn solve_station_positioning(map:&StationMap)->HashMap<String,Point>{
    const WIDTH: f32 = 500.0;
    const HEIGHT: f32 = 400.0;
    const LEFT:f32 = 80.0;
    const ITERATIONS: usize = 150;
    const INITIAL_TEMPERATURE: f32 = 80.0;
    const MIN_DISTANCE: f32 = 0.01;

    let mut names: Vec<String> = map.stations.keys().cloned().collect();
    names.sort();

    let count = names.len();
    if count == 0 {
        return HashMap::new();
    }

    // Assign each station a stable index.
    let indices: HashMap<&str, usize> = names
        .iter()
        .enumerate()
        .map(|(i, name)| (name.as_str(), i))
        .collect();

    // Store positions and displacement vectors by index.
    let mut positions: Vec<(f32, f32)> = (0..count)
        .map(|i| {
            let angle = std::f32::consts::TAU * i as f32
                / count as f32;
            let radius = 0.35 * WIDTH.min(HEIGHT);

            (
                WIDTH / 2.0 + radius * angle.cos(),
                HEIGHT / 2.0 + radius * angle.sin(),
            )
        })
        .collect();

    let mut displacement = vec![(0.0_f32, 0.0_f32); count];

    // Convert connections into indexed edges.
    let edges: Vec<(usize, usize)> = map
        .connections
        .iter()
        .filter_map(|connection| {
            let from = *indices.get(connection.from.as_str())?;
            let to = *indices.get(connection.to.as_str())?;

            if from == to {
                return None;
            }

            Some((from, to))
        })
        .collect();

    // Ideal spacing between stations.
    let area = WIDTH * HEIGHT;
    let k = (area / count as f32).sqrt();

    for iteration in 0..ITERATIONS {
        displacement.fill((0.0, 0.0));

        // 1. Repulsion: every pair of stations pushes apart.
        for i in 0..count {
            for j in (i + 1)..count {
                let dx = positions[i].0 - positions[j].0;
                let dy = positions[i].1 - positions[j].1;

                let distance =
                    (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);

                // Unit direction multiplied by k² / distance.
                let force = k * k / (distance * distance);
                let fx = dx * force;
                let fy = dy * force;

                displacement[i].0 += fx;
                displacement[i].1 += fy;
                displacement[j].0 -= fx;
                displacement[j].1 -= fy;
            }
        }

        // 2. Attraction: connected stations pull together.
        for &(from, to) in &edges {
            let dx = positions[from].0 - positions[to].0;
            let dy = positions[from].1 - positions[to].1;

            let distance =
                (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);

            // Unit direction multiplied by distance² / k.
            let force = distance / k;
            let fx = dx * force;
            let fy = dy * force;

            displacement[from].0 -= fx;
            displacement[from].1 -= fy;
            displacement[to].0 += fx;
            displacement[to].1 += fy;
        }

        // 3. Limit movement and gradually cool the system.
        let temperature = INITIAL_TEMPERATURE
            * (1.0 - iteration as f32 / ITERATIONS as f32);

        for i in 0..count {
            let (dx, dy) = displacement[i];
            let magnitude = (dx * dx + dy * dy).sqrt();

            if magnitude > MIN_DISTANCE {
                let movement = magnitude.min(temperature);
                positions[i].0 += dx / magnitude * movement;
                positions[i].1 += dy / magnitude * movement;
            }

            // Keep the layout inside the drawing area.
            positions[i].0 =
                positions[i].0.clamp(20.0, WIDTH - 20.0);
            positions[i].1 =
                positions[i].1.clamp(20.0, HEIGHT - 20.0);
        }
    }

    names
        .into_iter()
        .zip(positions)
        .map(|(name, (x, y))| (name, Point { x:x+LEFT, y }))
        .collect()
}


fn draw_rotated_rect(
    painter: &egui::Painter,
    center: egui::Pos2,
    size: egui::Vec2,
    angle: f32,
    color: egui::Color32,
) {
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;

    let corners = [
        egui::vec2(-half_width, -half_height),
        egui::vec2(half_width, -half_height),
        egui::vec2(half_width, half_height),
        egui::vec2(-half_width, half_height),
    ];

    let points: Vec<egui::Pos2> = corners
        .iter()
        .map(|corner| {
            let rotated = egui::vec2(
                corner.x * angle.cos() - corner.y * angle.sin(),
                corner.x * angle.sin() + corner.y * angle.cos(),
            );

            center + rotated
        })
        .collect();

    painter.add(egui::Shape::convex_polygon(
        points,
        color,
        egui::Stroke::NONE,
    ));
}