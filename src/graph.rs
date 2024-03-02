#![allow(dead_code)]
use crate::graphic_engine::{Engine, Color};
use std::collections::HashMap;
use rand::prelude::*;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub color: Color,
}

pub struct Polygon {
    pub points: Vec<Point2D>,
}

pub struct Graph {
    pub engine: Engine,
    pub points: Vec<Point2D>,
}

impl Graph {
    pub fn new(engine: Engine) -> Graph {
        Graph {
            engine,
            points: Vec::new(),
        }
    }    

    /// Generates random points on the graph based on the specified count, color, and minimum distance.
    /// 
    /// # Arguments
    /// 
    /// * `count` - The number of points to generate.
    /// * `color` - The color of the points.
    /// * `min_distance` - The minimum distance between points.
    ///
    pub fn generate_points(&mut self, count: u16, size: usize,  color: Color, min_distance: i16) {
        for _i in 0..count {
            let mut x = thread_rng().gen_range((size * 2)..self.engine.width - size * 2);
            let mut y = thread_rng().gen_range((size * 2)..self.engine.height - size * 2);

            while self.too_close(x as i32, y as i32, min_distance as i32) {
                x = thread_rng().gen_range((size * 2)..self.engine.width - size * 2);
                y = thread_rng().gen_range((size * 2)..self.engine.height - size * 2);
            }

            self.points.push(Point2D { x: x.try_into().unwrap(), y: y.try_into().unwrap(), size: size as i32, color });
        }
    }

    fn too_close(&self, x: i32, y: i32, min_distance: i32) -> bool {
        for point in &self.points {
            if (point.x - x).abs() < min_distance && (point.y - y). abs() < min_distance {
                return true;
            }
        }
        false
    }

    pub fn create_polygon(&mut self) -> Vec<Point2D> {
        let locale_points = self.points.clone();
        let start_point = locale_points.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap();
        let mut angles_map = HashMap::new();
        for point in locale_points.iter() {
            let angle = Self::angle_between_points(start_point, point);
            angles_map.insert(point, angle);
        }
        let mut sorted_points = angles_map.into_iter().collect::<Vec<_>>();
        sorted_points.sort_by(|a, b| {
            let angle_cmp = a.1.partial_cmp(&b.1).unwrap();
            if angle_cmp == std::cmp::Ordering::Equal {
                let dist_a = (a.0.x - start_point.x).pow(2) + (a.0.y - start_point.y).pow(2);
                let dist_b = (b.0.x - start_point.x).pow(2) + (b.0.y - start_point.y).pow(2);
                dist_b.partial_cmp(&dist_a).unwrap()
            } else {
                angle_cmp
            }
        });
        

        let mut stack: Vec<Point2D> = Vec::new();
        for &point in sorted_points.iter() {
            while stack.len() > 1 && !Self::left_turn(&stack[stack.len() - 2], &stack[stack.len() - 1], point.0) {
               stack.pop();
            }
            stack.push(*point.0);
        }

        if let Some(&first_point) = stack.first() {
            stack.push(first_point);
        }

        stack
    }

    fn left_turn(p: &Point2D, q: &Point2D, r: &Point2D) -> bool {
        let epsilon = 1e-3; // Increased epsilon value
        (q.y as f64 - p.y as f64) * (r.x as f64 - q.x as f64) < (q.x as f64 - p.x as f64) * (r.y as f64 - q.y as f64) + epsilon
    }    
    
    
    fn angle_between_points(a: &Point2D, b: &Point2D) -> f32 {
        let (x0, y0) = (a.x, a.y);
        let (x, y) = (b.x, b.y);
        let dx = (x - x0) as f32;
        let dy = (y - y0) as f32;
        dy.atan2(dx).to_degrees()
    }
}

pub struct Edge {
    target: Node,
    weight: i32,
}

impl Edge {
    pub fn new(target: Node, weight: i32) -> Self {
        Self { target, weight }
    }
}

pub struct Node {
    value: i32,
    edges: LinkedHashSet<Edge>,
    parents: LinkedHashMap<Node, Edge>,
}

impl Node {
    pub fn new(value: i32, edges: LinkedHashSet<Edge>, parents: LinkedHashMap<Node, Edge>) -> Self {
        Self { value, edges, parents }
    }
}