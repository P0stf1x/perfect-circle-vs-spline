use crate::{HEIGHT, WIDTH, common::*};

pub fn flood_fill(buf: &mut Screen, x: usize, y: usize, color: Color) {
    if x >= WIDTH || y >= HEIGHT { return }

    let mut check_queue: Vec<(usize, usize)> = vec![(x, y)];
    let checked_color = buf.get_pixel(x, y);
    if color.get_colors() == checked_color.get_colors() { panic!("Checked color == flood fill color") };

    loop {
        // get pixel from queue
        if let Some(pixel) = check_queue.pop() {

            // fill current pixel
            buf.set_pixel(pixel.0, pixel.1, color);

            // check neighbours
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_x = (pixel.0 as isize) + dx;
                let new_y = (pixel.1 as isize) + dy;
                if new_x < 0 || new_y < 0 || new_x >= WIDTH as isize || new_y >= HEIGHT as isize { continue }
                if buf.get_pixel(new_x as usize, new_y as usize).get_colors() == checked_color.get_colors() {
                    check_queue.push((new_x as usize, new_y as usize));
                }
            }

        } else {
            break; // break if pop() returned None (queue ended)
        }
    }
}
