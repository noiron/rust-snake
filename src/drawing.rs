use piston_window::{math, rectangle, G2d};

pub fn draw_block(x: u32, y: u32, transform: math::Matrix2d, graphics: &mut G2d) {
    rectangle(
        [0.0, 1.0, 0.0, 1.0],
        [x as f64 * 20.0, y as f64 * 20.0, 20.0, 20.0],
        transform,
        graphics,
    );
}
