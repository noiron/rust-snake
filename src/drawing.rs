use piston_window::{ellipse, math, rectangle, types::Color, G2d};

use crate::BLOCK_SIZE;

pub const BACKGROUND_COLOR: Color = [1.0, 0.522, 0.522, 1.0]; // #ff8585
pub const SNAKE_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // black
pub const FOOD_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // black
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // black

pub fn draw_block(x: u32, y: u32, color: Color, transform: math::Matrix2d, graphics: &mut G2d) {
    rectangle(
        color,
        rectangle::square(
            (x * BLOCK_SIZE) as f64,
            (y * BLOCK_SIZE) as f64,
            BLOCK_SIZE as f64,
        ),
        transform,
        graphics,
    );
}

pub fn draw_circle(x: u32, y: u32, color: Color, transform: math::Matrix2d, graphics: &mut G2d) {
    ellipse(
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

pub fn draw_border(transform: math::Matrix2d, graphics: &mut G2d, width: u32, height: u32) {
    // up
    rectangle(
        BORDER_COLOR,
        [0.0, 0.0, (width * BLOCK_SIZE) as f64, BLOCK_SIZE as f64],
        transform,
        graphics,
    );
    // right
    rectangle(
        BORDER_COLOR,
        [
            ((width - 1) * BLOCK_SIZE) as f64,
            0.0,
            BLOCK_SIZE as f64,
            (height * BLOCK_SIZE) as f64,
        ],
        transform,
        graphics,
    );
    // bottom
    rectangle(
        BORDER_COLOR,
        [
            0.0,
            ((width - 1) * BLOCK_SIZE) as f64,
            (width * BLOCK_SIZE) as f64,
            BLOCK_SIZE as f64,
        ],
        transform,
        graphics,
    );
    // left
    rectangle(
        BORDER_COLOR,
        [0.0, 0.0, BLOCK_SIZE as f64, (height * BLOCK_SIZE) as f64],
        transform,
        graphics,
    );
}
