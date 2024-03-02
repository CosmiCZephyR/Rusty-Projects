use minifb::{Window, Key, WindowOptions, Scale, ScaleMode};
use std::thread::sleep;
use std::time::Duration;

mod graphic_engine;
mod graph;
mod stack;
use graph::Graph;
use graphic_engine::{Engine, Color};

const MIN_DISTANCE: i16 = 0x20;
#[allow(dead_code)]
const DEFAULT_SIZE: usize = 0x04;

fn main() {
    let mut window = create_window(200, 150);

    let mut engine = Engine::new(200, 150);
    let mut graph = Graph::new(engine.clone());
    graph.generate_points(12, 2, Color::White, MIN_DISTANCE);
    let polygon = graph.create_polygon();

    while !window.is_key_down(Key::Escape) {
        engine.draw_points(&graph.points);
        engine.draw_polygon(&polygon);
        window.update_with_buffer(&engine.buffer as &[u32], engine.width, engine.height).unwrap();
        sleep(Duration::from_millis(500));
    }    
}

/// Creates a new window with the specified width and height.
/// 
/// # Argumen
/// 
/// * `width` - The width of the window.
/// * `height` - The height of the window.
/// 
/// # Returns
/// 
/// A `Window` object representing the newly created window.
fn create_window(width: usize, height: usize) -> Window {
    Window::new(
        "Hyprland window",
        width,
        height,
        WindowOptions {
            resize: true,
            scale: Scale::X4,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        }
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    })
}