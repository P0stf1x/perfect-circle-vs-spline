use crate::{HEIGHT, WIDTH, common::*, is_inside_circle};

pub fn flood_fill(buf: &mut Screen, x: usize, y: usize, color: Color, limit_to_circle: bool) {
    let checked_color = buf.get_pixel(x, y);
    flood_fill_algorith(buf, x, y, color, checked_color, limit_to_circle, true, true);
}

fn flood_fill_algorith(buf: &mut Screen, x: usize, y: usize, color: Color, checked_color: Color, limit_to_circle: bool, check_above: bool, check_below: bool) -> (usize, usize) {
    if limit_to_circle && !is_inside_circle(x, y) { return (x, x) }

    let mut left_edge = x;
    while left_edge > 0 && (!limit_to_circle || is_inside_circle(left_edge, y)) {
        if buf.get_pixel_unchecked(left_edge-1, y) != checked_color { break }
        left_edge -= 1;
    }

    let mut right_edge = x;
    while right_edge < (WIDTH - 1) && (!limit_to_circle || is_inside_circle(right_edge, y)) {
        if buf.get_pixel_unchecked(right_edge+1, y) != checked_color { break }
        right_edge += 1;
    }

    buf.fill_line_unsafe(y, left_edge, right_edge, color);

    // check_above and check_below trick wouldn't work in general case, but in this case it works and saves some processing

    // check pixels above for same color
    if check_above && y > 0 {
        let mut i = left_edge;
        while i <= right_edge {
            if buf.get_pixel_unchecked(i, y-1) == checked_color {
                let (_, fill_up_to) = flood_fill_algorith(buf, i, y-1, color, checked_color, limit_to_circle, true, false);
                i = fill_up_to + 1;
            } else {
                i += 1;
            }
        }
    }

    // check pixels below for same color
    if check_below && y < HEIGHT-1 {
        let mut i = left_edge;
        while i <= right_edge {
            if buf.get_pixel_unchecked(i, y+1) == checked_color {
                let (_, fill_up_to) = flood_fill_algorith(buf, i, y+1, color, checked_color, limit_to_circle, false, true);
                i = fill_up_to + 1;
            } else {
                i += 1;
            }
        }
    }

    return (left_edge, right_edge);
}
