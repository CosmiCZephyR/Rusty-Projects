#![allow(dead_code)]
use rusttype::{Font, Scale, point};
use std::thread::sleep;
use std::time::Duration;
use crate::graph::Point2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
}

impl Color {
    pub fn to_u32(&self) -> u32 {
        match self {
            Color::Black => 0x000000,
            Color::Red => 0xFF0000,
            Color::Green => 0x00FF00,
            Color::Blue => 0x0000FF,
            Color::Yellow => 0xFFFF00,
            Color::Cyan => 0x00FFFF,
            Color::Magenta => 0xFF00FF,
            Color::White => 0xFFFFFF,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Engine {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Engine {
    pub fn new(width: usize, height: usize) -> Engine {
        Engine {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn draw_polygon(&mut self, points: &Vec<Point2D>) {
        for i in 0..points.len() {
            if i < points.len() - 1 {
                self.draw_point(points[i].x, points[i].y, points[i].size as i32, points[i].color.to_u32());
                self.draw_point(points[i + 1].x, points[i + 1].y, points[i + 1].size as i32, points[i + 1].color.to_u32());
                self.draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, points[i].color.to_u32());
                sleep(Duration::from_millis(500));
            }
        }
    }

    pub fn draw_points(&mut self, points: &Vec<Point2D>) {
        for point in points.iter() {
            self.draw_point(point.x, point.y, point.size as i32, point.color.to_u32());
        }
    }

    pub fn draw_point(&mut self, x: i32, y: i32, size: i32, col: u32) {
        self.fill_square(x, y, x + size, y + size, col)
    }

    pub fn draw_square(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: u32) {
        self.draw_triangle(x1, y1, x2, y1, x2, y2, col);
        self.draw_triangle(x1, y1, x1, y2, x2, y2, col);
    }

    pub fn fill_square(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: u32) {
        self.fill_triangle(x1, y1, x2, y1, x2, y2, col);
        self.fill_triangle(x1, y1, x1, y2, x2, y2, col);
    }

    pub fn draw(&mut self, x: i32, y: i32, col: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.buffer[(y * self.width as i32 + x) as usize] = col;
        }
    }
 
    pub fn draw_circle(&mut self, xc: i32, yc: i32, r: i32) {
        let mut x: i32 = 0;
        let mut y: i32 = r;
        let mut p: i32 = 3 - 2 * r;
        if r == 0 {
            return;
        }

        while y >= x {
            self.draw(xc + x, yc + y, 0xFFFFFF);
            self.draw(xc + y, yc + x, 0xFFFFFF);
            self.draw(xc - y, yc + x, 0xFFFFFF);
            self.draw(xc - x, yc + y, 0xFFFFFF);
            self.draw(xc - x, yc - y, 0xFFFFFF);
            self.draw(xc - y, yc - x, 0xFFFFFF);
            self.draw(xc + y, yc - x, 0xFFFFFF);
            self.draw(xc + x, yc - y, 0xFFFFFF);
            if p < 0 {
                p += 4 * x + 6;
            } else {
                y -= 1;
                p += 4 * (x - y) + 10;
            }
            x += 1;
        }
    }

    pub fn fill_circle(&mut self, xc: i32, yc: i32, r: i32, col: u32) {
        let mut x: i32 = 0;
        let mut y: i32 = r;
        let mut p: i32 = 3 - 2 * r;
        if r == 0 {
            return;
        }

        while y >= x {
            self.fill_line(xc - x, xc + x, yc - y, col);
            self.fill_line(xc - y, xc + y, yc - x, col);
            self.fill_line(xc - x, xc + x, yc + y, col);
            self.fill_line(xc - y, xc + y, yc + x, col);
            if p < 0 {
                p += 4 * x + 6;
                x += 1;
            }
            else {
                p += 4 * (x - y) + 10;
                x += 1;
                y -= 1;
            }
        }
    }

    pub fn draw_triangle(&mut self ,x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: u32) {
        self.draw_line(x1, y1, x2, y2, col);
        self.draw_line(x2, y2, x3, y3, col);
        self.draw_line(x3, y3, x1, y1, col);
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: u32) {
        let (mut x, mut y, dx, dy, dx1, dy1, mut px, mut py): (i32, i32, i32, i32, i32, i32, i32, i32);
        dx = x2 - x1; dy = y2 - y1;
        dx1 = dx.abs(); dy1 = dy.abs();
        px = 2 * dy1 - dx1; py = 2 * dx1 - dy1;
        if dy1 <= dx1 {
            if dx >= 0 {
                x = x1; y = y1;
            } else {
                x = x2; y = y2;
            }
            self.draw(x, y, col);
            for _i in 0..dx1 {
                if px < 0 {
                    px = px + 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                    px = px + 2 * (dy1 - dx1);
                }
                x += 1;
                self.draw(x, y, col);
            }
        } else {
            if dy >= 0 {
                x = x1; y = y1;
            } else {
                x = x2; y = y2;
            }
            self.draw(x, y, col);
            for _i in 0..dy1 {
                if py <= 0 {
                    py = py + 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    py = py + 2 * (dx1 - dy1);
                }
                y += 1;
                self.draw(x, y, col);
            }
        }
    }


    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: u32) {
        // Sort the points by y-coordinate
        let mut points = [(x1, y1), (x2, y2), (x3, y3)];
        points.sort_by_key(|p| p.1);

        let (x1, y1) = points[0];
        let (x2, y2) = points[1];
        let (x3, y3) = points[2];

        // Calculate the slopes
        let slope_a = if y2 - y1 != 0 { (x2 - x1) as f32 / (y2 - y1) as f32 } else { 0.0 };
        let slope_b = if y3 - y1 != 0 { (x3 - x1) as f32 / (y3 - y1) as f32 } else { 0.0 };
        let slope_c = if y3 - y2 != 0 { (x3 - x2) as f32 / (y3 - y2) as f32 } else { 0.0 };

        // Draw the triangle
        for y in y1..=y2 {
            let xa = x1 as f32 + slope_a * (y - y1) as f32;
            let xb = x1 as f32 + slope_b * (y - y1) as f32;
            self.fill_line(xa.round() as i32, xb.round() as i32, y, col);
        }
        for y in y2..=y3 {
            let xa = x2 as f32 + slope_c * (y - y2) as f32;
            let xb = x1 as f32 + slope_b * (y - y1) as f32;
            self.fill_line(xa.round() as i32, xb.round() as i32, y, col);
        }
    }


    fn fill_line(&mut self, mut sx: i32, mut ex: i32, ny: i32, col: u32) {
        if sx > ex {
            Self::swap(&mut sx, &mut ex);
        }
        for x in sx..=ex {
            self.draw(x, ny, col);
        }
    }


    fn swap(x: &mut i32, y: &mut i32) {
        std::mem::swap(x, y);
    }

    #[allow(unused_assignments)]
    fn clip(&self, mut x: i32, mut y: i32) {
        if x < 0 {
            x = 0;
        }
        if x >= self.width as i32 {
            x = self.width as i32 - 1;
        }
        if y < 0 {
            y = 0;
        }
        if y >= self.height as i32 {
            y = self.height as i32 - 1;
        }
    }

    pub fn fill(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: u32) {
        self.clip(x1, y1);
        self.clip(x2, y2);
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.draw(x, y, col);
            }
        }
    }

    pub fn draw_string(&mut self, x: i32, y: i32, string: &str, col: u32) {
        let font: Font<'static> = Font::try_from_bytes(include_bytes!(r"assets/pixelfont.ttf") as &[u8]).unwrap();
        let height: f32 = 20f32; // adjust as needed
        let scale = Scale { x: height, y: height };
        let v_metrics = font.v_metrics(scale);
        let offset = point(x as f32, v_metrics.ascent + y as f32);
        let iter = font.layout(string, scale, offset);

        for g in iter {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {

                    let v = v * 0xFF as f32;
                    let x = x + bb.min.x as u32;
                    let y = y + bb.min.y as u32;
                    if v > 150.0 {
                        self.draw(x as i32, y as i32, col );
                    }
                });
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> u32 {
        self.buffer[y as usize * self.width + x as usize]
    }
}