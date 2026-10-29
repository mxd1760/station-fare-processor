use eframe::egui::{self, Painter};

pub type StationId = String;

#[derive(Clone)]
pub enum DrawingPath {
    Line {
        start: (StationId, Point),
        end: (StationId, Point),
        width: f32,
        color: Option<egui::Color32>,
    },
    //Bezier
    //PolyLine
    //Complex Curve
}

#[derive(Clone)]
pub enum Element {
    Path(DrawingPath),
    Circle {
        center: Point,
        radius: f32,
        color: Option<egui::Color32>,
        name: StationId,
    },
    Text {
        position: Point,
        text: String,
        color: Option<egui::Color32>,
    },
}

impl Element {
    pub fn draw(&self, painter: &Painter, rect: &egui::Rect, font: &egui::FontId) {
        match self {
            Element::Path(p) => match p {
                DrawingPath::Line {
                    start,
                    end,
                    width,
                    color,
                } => {
                    let track = egui::Stroke::new(
                        *width,
                        match color {
                            Some(v) => *v,
                            None => egui::Color32::LIGHT_GRAY,
                        },
                    );
                    painter.line_segment([start.1.to_pos2(*rect), end.1.to_pos2(*rect)], track);
                }
            },
            Element::Circle {
                center,
                radius,
                color,
                name: _,
            } => {
                let real_color = match color {
                    Some(c) => c,
                    None => &egui::Color32::LIGHT_GRAY,
                };
                painter.circle_filled(center.to_pos2(*rect), *radius, *real_color);
            }
            Element::Text {
                position,
                text,
                color,
            } => {
                painter.text(
                    position.to_pos2(*rect),
                    egui::Align2::CENTER_CENTER,
                    text.as_str(),
                    font.clone(),
                    match color {
                        Some(v) => *v,
                        None => egui::Color32::WHITE,
                    },
                );
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn plus(&mut self, other: &Self) -> Self {
        self.x += other.x;
        self.y += other.y;
        *self
    }
    pub fn minus(&mut self, other: &Self) -> Self {
        self.x -= other.x;
        self.y -= other.y;
        *self
    }
    pub fn to_pos2(&self, rect: egui::Rect) -> egui::Pos2 {
        rect.left_top() + egui::vec2(self.x, self.y)
    }
    pub fn times(&mut self, other: &f32) -> Self {
        self.x *= other;
        self.y *= other;
        *self
    }
}
