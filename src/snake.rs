use std::collections::LinkedList;

use piston_window::{Context, G2d};

use crate::drawing::draw_block;

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
            draw_block(block.x, block.y, context.transform, graphics);
        }
    }
}
