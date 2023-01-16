use piston_window::{math, rectangle, types::Color, G2d};

use crate::BLOCK_SIZE;

pub fn draw_block(x: u32, y: u32, color: Color, transform: math::Matrix2d, graphics: &mut G2d) {
    rectangle(
        color,
        [
            (x * BLOCK_SIZE) as f64,
            (y * BLOCK_SIZE) as f64,
            BLOCK_SIZE as f64,
            BLOCK_SIZE as f64,
        ],
        transform,
        graphics,
    );
}
