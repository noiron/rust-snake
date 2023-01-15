use piston_window::{types::Color, Context, G2d, Key};
use rand::Rng;

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
    width: u32,
    height: u32,
    snake: Snake,
    has_food: bool,
    food_x: u32,
    food_y: u32,
    waiting_time: f64,
    direction: Direction,
}

impl Game {
    pub fn new() -> Game {
        Game {
            width: 20,
            height: 20,
            snake: Snake::new(),
            has_food: true,
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

        if !self.has_food {
            self.add_food();
        }

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
            self.has_food = false;
        }
    }

    pub fn add_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x: u32 = rng.gen_range(0..self.width);
        let y: u32 = rng.gen_range(0..self.height);
        // TODO: 需要保证新位置不会和 body 重叠
        self.food_x = x;
        self.food_y = y;
        self.has_food = true;
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
