static CIRCLE_RADIUS: f64 = (WIDTH as f64) * CIRCLE_SIZE * 0.5;
static SPLINE_NODES: usize = 8;
pub static WIDTH:  usize = 512;
pub static HEIGHT: usize = 512;
pub static CIRCLE_SIZE: f64 = 0.8;
pub static CIRCLE_CENTER: Vec2 = Vec2 { x: (WIDTH as f64)/2., y: (HEIGHT as f64)/2. };
static BEZIER_QUALITY: usize = 126;

mod common;
mod text;
mod line;
mod bezier;
mod flood_fill;

use std::f64::consts::TAU;

use crate::common::*;
use crate::text::render_text;
use crate::line::*;
use crate::bezier::*;
use crate::flood_fill::*;

fn main() {
    let mut screen = Screen::new();
    render_text(&mut screen, "The quick brown fox jumps over the lazy dog\n1234567890\ntest: 0.42f".to_string(), 10, 10, 2, Color::new(255, 255, 0));

    let circle_part_rad = TAU/SPLINE_NODES as f64;
    let mut spline_points = Vec::<Vec2>::with_capacity(SPLINE_NODES);
    let mut spline_thetas = Vec::<f64>::with_capacity(SPLINE_NODES);
    for i in 0..SPLINE_NODES {
        let theta = (i as f64) * circle_part_rad;
        spline_thetas.push(theta);
        let v2 = pol2cart(CIRCLE_RADIUS, theta);
        let v2_offseted = v2.add(CIRCLE_CENTER);
        spline_points.push(v2_offseted);
    }
    let mut beziers = Vec::<Bezier>::with_capacity(SPLINE_NODES);
    for i in 0..SPLINE_NODES-1 {
        beziers.push(
            Bezier::from_line(spline_points[i], spline_points[i+1])
        );
    }
    beziers.push(
        Bezier::from_line(spline_points[SPLINE_NODES-1], spline_points[0])
    );
    // render_connected_lines(&mut screen, spline_points, true, false);
    // render_connected_beziers(&mut screen, beziers);
    let test_bez = Bezier {

        p0: Vec2 { x: -150., y:  25.},
        p1: Vec2 { x:  -50., y: -75.},
        p2: Vec2 { x:   50., y:  75.},
        p3: Vec2 { x:  150., y: -25.},
    };
    let mut circleish_beziers = Vec::<Bezier>::with_capacity(SPLINE_NODES);
    for i in 0..spline_thetas.len()-1 {
        circleish_beziers.push(
            Bezier::from_tangent(spline_thetas[i], spline_thetas[i+1], 1.1, CIRCLE_RADIUS, CIRCLE_CENTER)
        );
    }
    circleish_beziers.push(
        Bezier::from_tangent(spline_thetas[SPLINE_NODES-1], spline_thetas[0], 1.1, CIRCLE_RADIUS, CIRCLE_CENTER)
    );
    render_connected_beziers(&mut screen, circleish_beziers, Color::new(0, 255, 0));
    perfect_circle(&mut screen, Color::new(255, 0, 0));
    render_connected_beziers(&mut screen, beziers, Color::new(0, 0, 255));
    // render_bezier(&mut screen, test_bez.offset(CIRCLE_CENTER));
    // debug_bezier(&mut screen, test_bez.offset(CIRCLE_CENTER));
    flood_fill(&mut screen, WIDTH/2, HEIGHT/2, Color::new(255, 0, 255));

    screen.render_to_file("result.ppm".to_string());
}

fn perfect_circle(buf: &mut Screen, color: Color) {
    for y in 0..(HEIGHT as isize) {
        for x in 0..(WIDTH as isize) {
            let inside = (((x-((WIDTH as isize)/2)).pow(2) + (y-((HEIGHT as isize)/2)).pow(2)) as f64).sqrt() < CIRCLE_RADIUS;
            if inside {
                buf.set_pixel(x as usize, y as usize, color);
            };
        }
    }
}
