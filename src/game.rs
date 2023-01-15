use piston_window::{Context, G2d};

use crate::snake::Snake;

pub struct Game {
    snake: Snake,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d) {
        self.snake.draw(context, graphics);
    }
}
