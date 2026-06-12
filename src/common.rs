use std::fs;
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

    pub fn render_to_file(&mut self, file_name: String) {
        static HEADER_MAGIC_SIZE: usize = 3;
        static HEADER_RESOLUTION_SIZE: usize = 16; // roughly
        let mut write_buf = Vec::<u8>::with_capacity(WIDTH*HEIGHT*4 + HEADER_MAGIC_SIZE + HEADER_RESOLUTION_SIZE);

        write_buf.extend_from_slice(b"P3\n"); // magic
        write_buf.extend_from_slice(format!("{} {}\n", WIDTH, HEIGHT).as_bytes()); // size
        write_buf.extend_from_slice(b"255\n"); // color depth

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let (r, g, b) = self.get_pixel(x, y).get_colors();
                write_buf.extend_from_slice(format!("{} ", r).as_bytes());
                write_buf.extend_from_slice(format!("{} ", g).as_bytes());
                write_buf.extend_from_slice(format!("{} ", b).as_bytes());
            }
            write_buf.extend_from_slice(b"\n");
        }

        let mut file_handle = fs::File::create(file_name).unwrap();
        file_handle.write(&write_buf).unwrap();
    }
}

pub fn pol2cart(r: f64, th: f64) -> Vec2 {
    return Vec2 {
        x: (r * th.cos()) as isize,
        y: (r * th.sin()) as isize,
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
