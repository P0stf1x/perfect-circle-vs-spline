static BEZIER_DEBUG_COLOR: Color = Color::new(255, 255, 255);
static BEZIER_DEBUG_COLOR_0: Color = Color::new(255, 0, 0);
static BEZIER_DEBUG_COLOR_1: Color = Color::new(128, 255, 0);
static BEZIER_DEBUG_COLOR_2: Color = Color::new(0, 255, 255);
static BEZIER_DEBUG_COLOR_3: Color = Color::new(127, 0, 255);

use crate::common::*;
use crate::line::*;
use crate::BEZIER_QUALITY;

#[allow(unused)]
pub fn debug_bezier(buf: &mut Screen, b: Bezier) {
    buf.set_pixel(b.p0.x as usize, b.p0.y as usize, BEZIER_DEBUG_COLOR_0);
    buf.set_pixel(b.p1.x as usize, b.p1.y as usize, BEZIER_DEBUG_COLOR_1);
    buf.set_pixel(b.p2.x as usize, b.p2.y as usize, BEZIER_DEBUG_COLOR_2);
    buf.set_pixel(b.p3.x as usize, b.p3.y as usize, BEZIER_DEBUG_COLOR_3);
    for point in b.get_bezier_points(BEZIER_QUALITY, true) {
        buf.set_pixel(point.x as usize, point.y as usize, BEZIER_DEBUG_COLOR);
    }
}

pub fn render_bezier(buf: &mut Screen, b: Bezier, color: Color) {
    let points = b.get_bezier_points(BEZIER_QUALITY, true);
    for point in &points {
        let x = point.x.round() as usize;
        let y = point.y.round() as usize;
        buf.set_pixel(x, y, color);
    }
    render_connected_lines(buf, points, false, false, color);
}

impl Bezier {
    pub fn get_bezier_points(&self, n: usize, include_p0_p3: bool) -> Vec<Vec2> {
        let mut result = Vec::<Vec2>::with_capacity(n + 2);
        if include_p0_p3 {
            result.push(self.p3);
        }
        // idk why this produces them in backwards order (p3 -> p2 -> p1 -> p0) even though I'm processing in normal order, but I don't really care
        if n > 0 {
            let t_offset = 1./((n + 1) as f64);
            for i in 1..=n {
                let t = t_offset*(i as f64);
                if t < 0. || t > 1. {panic!("t is not 0..=1: {t}")}

                // reduce to 3 points
                let i_0 = lerp(self.p0, self.p1, t);
                let i_1 = lerp(self.p1, self.p2, t);
                let i_2 = lerp(self.p2, self.p3, t);

                // reduce to 2 points
                let ii_0 = lerp(i_0, i_1, t);
                let ii_1 = lerp(i_1, i_2, t);

                // reduce to 1 point
                let iii = lerp(ii_0, ii_1, t);

                result.push(iii);
            }
        }
        if include_p0_p3 {
            result.push(self.p0);
        }
        return result;
    }
}

pub fn render_connected_beziers(buf: &mut Screen, points: &Vec<Bezier>, color: Color) {
    for i in 0..points.len() {
        render_bezier(buf, points[i], color);
    }
}
