use piston_window::{types::Color, Context, G2d, Key};

use crate::{drawing::draw_block, snake::Snake};

const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    snake: Snake,
    food_x: u32,
    food_y: u32,
    waiting_time: f64,
    direction: Direction,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            food_x: 8,
            food_y: 8,
            waiting_time: 0.0,
            direction: Direction::Right,
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d) {
        self.draw_food(context, graphics);
        self.snake.draw(context, graphics);
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

        if self.waiting_time > 0.2 {
            self.snake.update(self.direction);
            self.eat_food();
            self.waiting_time = 0.0;
        }
    }

    pub fn eat_food(&mut self) {
        let (head_x, head_y) = self.snake.head();
        if head_x == self.food_x && head_y == self.food_y {
            self.snake.eat();
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        // TODO: 和当前方向相反的按键无效

        match key {
            Key::Up => self.direction = Direction::Up,
            Key::Down => self.direction = Direction::Down,
            Key::Left => self.direction = Direction::Left,
            Key::Right => self.direction = Direction::Right,
            _ => (),
        }
    }
}
