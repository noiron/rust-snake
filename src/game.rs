use piston_window::{Context, G2d, types::Color};

use crate::{drawing::draw_block, snake::Snake};

const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 0.0];

pub struct Game {
    snake: Snake,
    food_x: u32,
    food_y: u32,
}


impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            food_x: 8,
            food_y: 8,
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d) {
        self.snake.draw(context, graphics);
        self.draw_food(context, graphics);
    }

    pub fn draw_food(&mut self, context: Context, graphics: &mut G2d) {
        draw_block(
            self.food_x,
            self.food_y,
            FOOD_COLOR,
            context.transform,
            graphics,
        )
    }
}
