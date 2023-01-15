use std::collections::LinkedList;

use piston_window::{types::Color, Context, G2d};

use crate::{drawing::draw_block, game::Direction};

const SNAKE_COLOR: Color = [0.0, 1.0, 0.0, 1.0];

struct Block {
    x: u32,
    y: u32,
}

impl Block {
    fn new(x: u32, y: u32) -> Block {
        Block { x, y }
    }
}

pub struct Snake {
    body: LinkedList<Block>,
}

impl Snake {
    pub fn new() -> Snake {
        let mut body = LinkedList::new();
        body.push_back(Block::new(6, 4));
        body.push_back(Block::new(5, 4));
        body.push_back(Block::new(4, 4));

        Snake { body }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d) {
        for block in self.body.iter() {
            draw_block(block.x, block.y, SNAKE_COLOR, context.transform, graphics);
        }
    }

    pub fn update(&mut self, dir: Direction) {
        let head = self.body.front().unwrap();

        let new_block: Block = match dir {
            Direction::Up => Block::new(head.x, head.y - 1),
            Direction::Down => Block::new(head.x, head.y + 1),
            Direction::Left => Block::new(head.x - 1, head.y),
            Direction::Right => Block::new(head.x + 1, head.y),
        };

        self.body.push_front(new_block);
        self.body.pop_back();
    }
}
