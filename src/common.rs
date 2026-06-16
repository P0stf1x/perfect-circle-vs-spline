use std::{f64::consts::PI, fs};
use std::io::Write;

use crate::{ WIDTH, HEIGHT, };

#[derive(Clone, Copy)]
pub struct Color([u8; 3]);

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(
            [r, g, b]
        )
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }

    pub fn get_colors(&self) -> (u8, u8, u8) {
        return (self.r(), self.g(), self.b());
    }
}

pub struct Screen(Vec<Color>);

impl Screen {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x > WIDTH || y > HEIGHT {panic!("tried setting pixel outside screen border: x:{x} y:{y}")};
        let i = y*WIDTH + x;
        self.0[i] = color;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Color {
        if x > WIDTH || y > HEIGHT {panic!()};
        let i = y*WIDTH + x;
        return self.0[i];
    }

    pub fn new() -> Self {
        Self(
            vec![Color::new(0, 0, 0); WIDTH * HEIGHT],
        )
    }

    pub fn fill(&mut self, color: Color) {
        self.0.fill(color);
    }

    pub fn render_to_file(&mut self, file_name: String) {
        static HEADER_MAGIC_SIZE: usize = 3;
        static HEADER_RESOLUTION_SIZE: usize = 16; // roughly
        let mut write_buf = Vec::<u8>::with_capacity(WIDTH*HEIGHT*4 + HEADER_MAGIC_SIZE + HEADER_RESOLUTION_SIZE);

        write_buf.extend_from_slice(b"P6\n"); // magic
        write_buf.extend_from_slice(format!("{} {}\n", WIDTH, HEIGHT).as_bytes()); // size
        write_buf.extend_from_slice(b"255\n"); // color depth

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                write_buf.push(self.get_pixel(x, y).r());
                write_buf.push(self.get_pixel(x, y).g());
                write_buf.push(self.get_pixel(x, y).b());
            }
        }

        fs::create_dir_all("output/").unwrap();
        let mut file_handle = fs::File::create(file_name).unwrap();
        file_handle.write(&write_buf).unwrap();
    }
}

pub fn pol2cart(r: f64, th: f64) -> Vec2 {
    return Vec2 {
        x: (r * th.cos()),
        y: (r * th.sin()),
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Bezier {
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

impl Bezier {
    pub fn offset(self, other: Vec2) -> Self {
        Self {
            p0: self.p0.add(other),
            p1: self.p1.add(other),
            p2: self.p2.add(other),
            p3: self.p3.add(other),
        }
    }

    pub fn from_line(p0: Vec2, p3: Vec2) -> Self {
        let p1 = lerp(p0, p3, 1./3.);
        let p2 = lerp(p0, p3, 2./3.);
        return Self { p0, p1, p2, p3 };
    }

    pub fn from_tangent(th_p0: f64, th_p3: f64, r_percent: f64, circle_r: f64, offset: Vec2) -> Self {
        let p0 = pol2cart(circle_r, th_p0);
        let p3 = pol2cart(circle_r, th_p3);
        let abs_r = r_percent * circle_r;
        if abs_r.abs() < f64::EPSILON { // f
            let result = Self::from_line(p0, p3);
            return result.offset(offset);
        } else {
            let r = circle_r * r_percent;
            let p1 = p0.add(circle_tangent_vectors(th_p0, r).0);
            let p2 = p3.add(circle_tangent_vectors(th_p3, r).1);
            let result = Self { p0, p1, p2, p3 };
            return result.offset(offset);
        }
    }
}

pub fn lerp(p0: Vec2, p1: Vec2, t: f64) -> Vec2 {
    let diff = p1.sub(p0);
    let result = Vec2 {
        x: p0.x + (diff.x as f64 * (1.-t)),
        y: p0.y + (diff.y as f64 * (1.-t)),
    };
    return result;
}

pub fn circle_tangent_vectors(th: f64, r: f64) -> (Vec2, Vec2) {
    let left_th = th + PI/2.;
    let right_th = th - PI/2.;
    let left_vector = pol2cart(r, left_th);
    let right_vector = pol2cart(r, right_th);
    return (left_vector, right_vector)
}
