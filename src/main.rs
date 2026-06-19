static CIRCLE_RADIUS: f64 = (WIDTH as f64) * CIRCLE_SIZE * 0.5;
static SPLINE_NODES: usize = 8;
pub static WIDTH:  usize = 512;
pub static HEIGHT: usize = 512;
pub static CIRCLE_SIZE: f64 = 0.8;
pub static CIRCLE_CENTER: Vec2 = Vec2 { x: (WIDTH as f64)/2., y: (HEIGHT as f64)/2. };
static BEZIER_QUALITY: usize = 126;
static FRAMERATE: f64 = 30.;
static DURATION: f64 = 30.;
static RADIUS_START: f64 = 0.0;
static RADIUS_END: f64 = 1.3;

mod common;
mod text;
mod line;
mod bezier;
mod flood_fill;

use std::f64::consts::TAU;
use std::fs;
use std::process::exit;

use crate::common::*;
use crate::text::render_text;
use crate::line::*;
use crate::bezier::*;
use crate::flood_fill::*;

pub fn render_frame_inplace(buf: &mut Screen, bezier_rounding_percent: f64) {
    // Common calculations
    let circle_part_rad = TAU/SPLINE_NODES as f64;
    let mut spline_points = Vec::<Vec2>::with_capacity(SPLINE_NODES);
    let mut spline_thetas = Vec::<f64>::with_capacity(SPLINE_NODES);
    for i in 0..SPLINE_NODES {
        let theta = (i as f64) * circle_part_rad;
        spline_thetas.push(theta);
        let v2 = pol2cart(&CIRCLE_RADIUS, &theta);
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
    let mut circleish_beziers = Vec::<Bezier>::with_capacity(SPLINE_NODES);
    for i in 0..spline_thetas.len()-1 {
        circleish_beziers.push(
            Bezier::from_tangent(spline_thetas[i], spline_thetas[i+1], bezier_rounding_percent, CIRCLE_RADIUS, CIRCLE_CENTER)
        );
    }
    circleish_beziers.push(
        Bezier::from_tangent(spline_thetas[SPLINE_NODES-1], spline_thetas[0], bezier_rounding_percent, CIRCLE_RADIUS, CIRCLE_CENTER)
    );

    // Outer layer
    render_connected_beziers(buf, &circleish_beziers, Color::new(255, 0, 0));
    flood_fill(buf, WIDTH/2, HEIGHT/2, Color::new(255, 0, 0), false);

    // Middle/circle layer
    render_perfect_circle(buf, Color::new(255, 0, 0));

    // Inner layer
    render_connected_beziers(buf, &circleish_beziers, Color::new(255, 255, 255));
    flood_fill(buf, WIDTH/2, HEIGHT/2, Color::new(255, 255, 255), true);
    render_connected_beziers(buf, &circleish_beziers, Color::new(255, 0, 0)); // re-render them in red to fix cases when bezier is larger than the circle

    // HUD
    let segment_point = &circleish_beziers[0].offset(CIRCLE_CENTER.neg()).get_bezier_points(BEZIER_QUALITY, false);
    let mut average_sum = 0.;
    for i in 0..segment_point.len() {
        let (distance, _) = cart2pol(&segment_point[i]);
        average_sum += distance;
    }
    let average = average_sum/(BEZIER_QUALITY as f64)/CIRCLE_RADIUS;
    let middle_point = &circleish_beziers[0].offset(CIRCLE_CENTER.neg()).get_bezier_points(1, false)[0];
    let (center_r, _) = cart2pol(middle_point);
    let center = center_r/(CIRCLE_RADIUS as f64);
    let hud_text = format!(
        "Bezier average radius: {}\nBezier center radius: {}\nCircle-ness: {:.2}% of radius",
        average, center, bezier_rounding_percent*100.
    );
    render_text(buf, hud_text, 10, 10, 4, Color::new(0, 255, 0));
}

fn main() {
    if let Ok(mut dir) = fs::read_dir("output/") {
        if dir.next().is_some() {
            println!("output/ directory is not empty!");
            exit(-1);
        }
    }
    let mut screen = Screen::new();

    let mut roundness_counter = RADIUS_START;
    let frames_to_render = (FRAMERATE*DURATION).ceil() as usize;
    let roundness_diff = (RADIUS_END-RADIUS_START)/(frames_to_render-1) as f64;
    let mut counter = 0;

    println!("Rendering {} images:", frames_to_render);

    while counter < frames_to_render {
        screen.fill(Color::new(0, 0, 0));
        render_frame_inplace(&mut screen, roundness_counter);
        screen.render_to_file(format!("output/output_{}.png", counter), RenderFormat::PNG);
        roundness_counter += roundness_diff;
        counter += 1;
        println!("{}/{}", counter, frames_to_render)
    }
}

fn render_perfect_circle(buf: &mut Screen, color: Color) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if is_inside_circle(x, y) {
                buf.set_pixel(x as usize, y as usize, color);
            };
        }
    }
}

pub fn is_inside_circle(x: usize, y: usize) -> bool {
    let x_sq = ((x as isize)-((WIDTH as isize)/2)).pow(2);
    let y_sq = ((y as isize)-((HEIGHT as isize)/2)).pow(2);
    let inside = ((x_sq + y_sq) as f64).sqrt() < CIRCLE_RADIUS;
    return inside;
}
