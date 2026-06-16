use phf::phf_map;

use crate::common::*;

pub fn render_text(buf: &mut Screen, text: String, x: usize, y: usize, scale: usize, color: Color) {
    let mut x_pos = 0;
    let mut y_pos = 0;
    for char in text.chars() {
        render_symbol(buf, Symbol::new(char), x + (x_pos * 4 * scale), y + (y_pos * 6 * scale), scale, color);
        if char == '\n' {
            x_pos = 0;
            y_pos += 1;
        } else {
            x_pos += 1;
        }
    }
}

fn render_symbol(buf: &mut Screen, symbol: Symbol, x: usize, y: usize, scale: usize, color: Color) {
    for rel_y in 0..(5*scale) {
        let pix_y = rel_y/scale; // it's unoptimal to multiply and then right afterwards divide, but that makes code easier which is better for a quick one off project
        let abs_y = rel_y + y;
        for rel_x in 0..(3*scale) {
            let pix_x = rel_x/scale;
            let abs_x = rel_x + x;
            if symbol.at(pix_x, pix_y) {
                buf.set_pixel(abs_x, abs_y, color);
            }
        }
    }
}

type SymbolData = [bool; 3*5];

struct Symbol(SymbolData);

impl Symbol {
    fn new(char: char) -> Self {
        let lowercase_char = char.to_lowercase().next().unwrap();
        Symbol(
            if SYMBOL_ENCODINGS.contains_key(&lowercase_char) {
                SYMBOL_ENCODINGS.get(&lowercase_char).unwrap().clone()
            } else {
                SYMBOL_ENCODINGS.get(&' ').unwrap().clone()
            }
        )
    }

    fn at(&self, x: usize, y: usize) -> bool {
        if x >= 3 || y >= 5 {panic!("pixel outbounds: x:{x} y:{y}")}
        return self.0[x + y*3];
    }
}

static SYMBOL_ENCODINGS: phf::Map<char, SymbolData> = phf_map! {
    'a' => [  true,  true,  true,    true, false,  true,    true,  true,  true,    true, false,  true,    true, false,  true ],
    'b' => [  true,  true,  true,    true, false,  true,    true,  true, false,    true, false,  true,    true,  true,  true ],
    'c' => [  true,  true,  true,    true, false, false,    true, false, false,    true, false, false,    true,  true,  true ],
    'd' => [  true,  true, false,    true, false,  true,    true, false,  true,    true, false,  true,    true,  true, false ],
    'e' => [  true,  true,  true,    true, false, false,    true,  true, false,    true, false, false,    true,  true,  true ],
    'f' => [  true,  true,  true,    true, false, false,    true,  true, false,    true, false, false,    true, false, false ],
    'g' => [  true,  true,  true,    true, false, false,    true, false,  true,    true, false,  true,    true,  true,  true ],
    'h' => [  true, false,  true,    true, false,  true,    true,  true,  true,    true, false,  true,    true, false,  true ],
    'i' => [  true,  true,  true,   false,  true, false,   false,  true, false,   false,  true, false,    true,  true,  true ],
    'j' => [ false, false,  true,   false, false,  true,   false, false,  true,    true, false,  true,    true,  true,  true ],
    'k' => [  true, false,  true,    true, false,  true,    true,  true, false,    true, false,  true,    true, false,  true ],
    'l' => [  true, false, false,    true, false, false,    true, false, false,    true, false, false,    true,  true,  true ],
    'm' => [  true, false,  true,    true,  true,  true,    true, false,  true,    true, false,  true,    true, false,  true ],
    'n' => [ false, false,  true,    true, false,  true,    true,  true,  true,    true, false,  true,    true, false, false ],
    'o' => [  true,  true,  true,    true, false,  true,    true, false,  true,    true, false,  true,    true,  true,  true ],
    'p' => [  true,  true,  true,    true, false,  true,    true,  true,  true,    true, false, false,    true, false, false ],
    'q' => [  true,  true,  true,    true, false,  true,    true, false,  true,    true,  true,  true,   false, false,  true ],
    'r' => [  true,  true, false,    true, false,  true,    true,  true, false,    true, false,  true,    true, false,  true ],
    's' => [  true,  true,  true,    true, false, false,    true,  true,  true,   false, false,  true,    true,  true,  true ],
    't' => [  true,  true,  true,   false,  true, false,   false,  true, false,   false,  true, false,   false,  true, false ],
    'u' => [  true, false,  true,    true, false,  true,    true, false,  true,    true, false,  true,    true,  true,  true ],
    'v' => [  true, false,  true,    true, false,  true,    true, false,  true,    true, false,  true,   false,  true, false ],
    'w' => [  true, false,  true,    true, false,  true,    true, false,  true,    true,  true,  true,    true, false,  true ],
    'x' => [  true, false,  true,    true, false,  true,   false,  true, false,    true, false,  true,    true, false,  true ],
    'y' => [  true, false,  true,    true, false,  true,   false,  true, false,   false,  true, false,   false,  true, false ],
    'z' => [  true,  true,  true,   false, false,  true,   false,  true, false,    true, false, false,    true,  true,  true ],
    '0' => [  true,  true,  true,    true, false,  true,    true, false,  true,    true, false,  true,    true,  true,  true ],
    '1' => [ false,  true, false,    true,  true, false,   false,  true, false,   false,  true, false,    true,  true,  true ],
    '2' => [  true,  true,  true,   false, false,  true,    true,  true,  true,    true, false, false,    true,  true,  true ],
    '3' => [  true,  true,  true,   false, false,  true,   false,  true,  true,   false, false,  true,    true,  true,  true ],
    '4' => [  true, false,  true,    true, false,  true,    true,  true,  true,   false, false,  true,   false, false,  true ],
    '5' => [  true,  true,  true,    true, false, false,    true,  true,  true,   false, false,  true,    true,  true,  true ],
    '6' => [  true,  true,  true,    true, false, false,    true,  true,  true,    true, false,  true,    true,  true,  true ],
    '7' => [  true,  true,  true,   false, false,  true,   false,  true, false,   false,  true, false,   false,  true, false ],
    '8' => [  true,  true,  true,    true, false,  true,    true,  true,  true,    true, false,  true,    true,  true,  true ],
    '9' => [  true,  true,  true,    true, false,  true,    true,  true,  true,   false, false,  true,    true,  true,  true ],
    ':' => [ false, false, false,    true, false, false,   false, false, false,    true, false, false,   false, false, false ],
    '.' => [ false, false, false,   false, false, false,   false, false, false,   false, false, false,   false,  true, false ],
    ' ' => [ false, false, false,   false, false, false,   false, false, false,   false, false, false,   false, false, false ],
};
