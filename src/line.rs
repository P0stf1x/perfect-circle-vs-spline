static LINE_COLOR: Color = Color::new(255, 0, 0);

use crate::common::*;

pub fn render_line(buf: &mut Screen, p0: Vec2, p1: Vec2) { // Pretty much direct translation from https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
    if (p1.y - p0.y).abs() < (p1.x - p0.x).abs() {
        if p0.x > p1.x {
            render_line_low(buf, p1, p0)
        } else {
            render_line_low(buf, p0, p1)
        }
    } else {
        if p0.y > p1.y {
            render_line_high(buf, p1, p0)
        } else {
            render_line_high(buf, p0, p1)
        }
    }
}

fn render_line_low(buf: &mut Screen, p0: Vec2, p1: Vec2) {
    let dx = p1.x - p0.x;
    let mut dy = p1.y - p0.y;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = p0.y;

    for x in p0.x..p1.x {
        buf.set_pixel(x as usize, y as usize, LINE_COLOR);
        if d > 0 {
            y = y + yi;
            d = d + (2 * (dy - dx));
        } else {
            d = d + 2*dy;
        }
    }
}

fn render_line_high(buf: &mut Screen, p0: Vec2, p1: Vec2) {
    let mut dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = (2 * dx) - dy;
    let mut x = p0.x;

    for y in p0.y..p1.y {
        buf.set_pixel(x as usize, y as usize, LINE_COLOR);
        if d > 0 {
            x = x + xi;
            d = d + (2 * (dx - dy));
        } else {
            d = d + 2*dx;
        }
    }
}

pub fn render_connected_lines(buf: &mut Screen, points: Vec<Vec2>) {
    for i in 0..points.len()-1 {
        render_line(buf, points[i], points[i+1]);
    }
    render_line(buf, points[points.len()-1], points[0]);
}
