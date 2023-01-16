use piston_window::{Context, G2d, Key};
use rand::Rng;

use crate::{
    drawing::{draw_circle, FOOD_COLOR},
    snake::Snake,
};

const INTERVAL: f64 = 0.2;

#[derive(Clone, Copy, PartialEq)]
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
    game_over: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            width,
            height,
            snake: Snake::new(),
            has_food: true,
            food_x: 8,
            food_y: 8,
            waiting_time: 0.0,
            direction: Direction::Right,
            game_over: false,
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d) {
        // draw_border(context.transform, graphics, self.width, self.height);
        self.draw_food(context, graphics);
        self.snake.draw(context, graphics);
    }

    fn draw_food(&mut self, context: Context, graphics: &mut G2d) {
        draw_circle(
            self.food_x,
            self.food_y,
            FOOD_COLOR,
            context.transform,
            graphics,
        )
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            return;
        }

        if !self.has_food {
            self.add_food();
        }

        if self.waiting_time > INTERVAL {
            // 先检查再实际更新
            if self
                .snake
                .check_alive(self.direction, self.width, self.height)
            {
                self.snake.update(self.direction);
                self.eat_food();
                self.waiting_time = 0.0;
            } else {
                self.game_over = true;
            }
        }
    }

    fn eat_food(&mut self) {
        let (head_x, head_y) = self.snake.head();
        if head_x == self.food_x && head_y == self.food_y {
            self.snake.eat();
            self.has_food = false;
        }
    }

    fn add_food(&mut self) {
        let mut rng = rand::thread_rng();
        let mut x: u32 = rng.gen_range(0..self.width);
        let mut y: u32 = rng.gen_range(0..self.height);
        // 只会在没有 food 的时候才会添加 food，不会出现 snake 在吃了 food 之后占用 tail
        // 位置的情况。tail 这个位置在下一刻是一定会空出来的，可以用于添加新的 food。
        if self.snake.is_overlap_except_tail(x, y) {
            x = rng.gen_range(0..self.width);
            y = rng.gen_range(0..self.height);
        }
        self.food_x = x;
        self.food_y = y;
        self.has_food = true;
    }

    fn restart(&mut self) {
        self.snake = Snake::new();
        self.add_food();
        self.waiting_time = 0.0; 
        self.direction = Direction::Right;
        self.game_over = false;
    }

    pub fn key_pressed(&mut self, key: Key) {
        // TODO: 和当前方向相反的按键无效

        match key {
            Key::Up => self.direction = Direction::Up,
            Key::Down => self.direction = Direction::Down,
            Key::Left => self.direction = Direction::Left,
            Key::Right => self.direction = Direction::Right,
            Key::R => {
                self.restart();
            },
            _ => (),
        }
    }
}
