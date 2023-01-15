use piston_window::{math, rectangle, G2d, types::Color};

pub fn draw_block(x: u32, y: u32, color: Color, transform: math::Matrix2d, graphics: &mut G2d) {
    rectangle(
        color,
        [x as f64 * 20.0, y as f64 * 20.0, 20.0, 20.0],
        transform,
        graphics,
    );
}
