use std::collections::HashMap;

use eframe::egui;

use crate::{map::StationMap, simulator::drawable_elements::Point};

// TODO: BENCHMARK ME
pub fn solve_station_positioning(map: &StationMap) -> HashMap<String, Point> {
    const WIDTH: f32 = 500.0;
    const HEIGHT: f32 = 400.0;
    const LEFT: f32 = 80.0;
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
            let angle = std::f32::consts::TAU * i as f32 / count as f32;
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

                let distance = (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);

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

            let distance = (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);

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
        let temperature = INITIAL_TEMPERATURE * (1.0 - iteration as f32 / ITERATIONS as f32);

        for i in 0..count {
            let (dx, dy) = displacement[i];
            let magnitude = (dx * dx + dy * dy).sqrt();

            if magnitude > MIN_DISTANCE {
                let movement = magnitude.min(temperature);
                positions[i].0 += dx / magnitude * movement;
                positions[i].1 += dy / magnitude * movement;
            }

            // Keep the layout inside the drawing area.
            positions[i].0 = positions[i].0.clamp(20.0, WIDTH - 20.0);
            positions[i].1 = positions[i].1.clamp(20.0, HEIGHT - 20.0);
        }
    }

    names
        .into_iter()
        .zip(positions)
        .map(|(name, (x, y))| (name, Point { x: x + LEFT, y }))
        .collect()
}

pub fn draw_rotated_rect(
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
