use piston_window::{types::Color, Context, G2d};

use crate::{drawing::draw_block, snake::Snake};

const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

pub struct Game {
    snake: Snake,
    food_x: u32,
    food_y: u32,
    waiting_time: f64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            food_x: 8,
            food_y: 8,
            waiting_time: 0.0,
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

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.waiting_time > 1.0 {
            self.snake.update();
            self.waiting_time = 0.0;
        }
    }
}
