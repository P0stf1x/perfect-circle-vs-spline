static CIRCLE_RADIUS: f64 = (WIDTH as f64) * CIRCLE_SIZE * 0.5;
static SPLINE_NODES: usize = 8;
pub static WIDTH:  usize = 512;
pub static HEIGHT: usize = 512;
pub static CIRCLE_SIZE: f64 = 0.8;

mod common;
mod text;
mod line;

use std::f64::consts::TAU;

use crate::common::*;
use crate::text::render_text;
use crate::line::*;

fn main() {
    let mut screen = Screen::new();
    perfect_circle(&mut screen);
    render_text(&mut screen, "The quick brown fox jumps over the lazy dog\n1234567890\ntest: 0.42f".to_string(), 10, 10, 2);

    let circle_part_rad = TAU/SPLINE_NODES as f64;
    let mut spline_points = Vec::<Vec2>::with_capacity(SPLINE_NODES);
    for i in 0..SPLINE_NODES {
        let v2 = pol2cart(CIRCLE_RADIUS, (i as f64) * circle_part_rad);
        let v2_offseted = v2.add(Vec2 { x: (WIDTH/2) as isize, y: (HEIGHT/2) as isize });
        spline_points.push(v2_offseted);
    }
    render_connected_lines(&mut screen, spline_points);

    screen.render_to_file("result.ppm".to_string());
}

fn perfect_circle(buf: &mut Screen) {
    for y in 0..(HEIGHT as isize) {
        for x in 0..(WIDTH as isize) {
            let inside = (((x-((WIDTH as isize)/2)).pow(2) + (y-((HEIGHT as isize)/2)).pow(2)) as f64).sqrt() < CIRCLE_RADIUS;
            let color = if inside {
                Color::new(255, 255, 255)
            } else {
                Color::new(0, 0, 0)
            };
            buf.set_pixel(x as usize, y as usize, color);
        }
    }
}
