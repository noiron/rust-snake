use std::collections::LinkedList;

use piston_window::{types::Color, Context, G2d};

use crate::{drawing::draw_block, game::Direction};

const SNAKE_COLOR: Color = [0.0, 1.0, 0.0, 1.0];

#[derive(Clone, Copy)]
pub struct Block {
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
    last_removed_block: Option<Block>,
}

impl Snake {
    pub fn new() -> Snake {
        let mut body = LinkedList::new();
        body.push_back(Block::new(6, 4));
        body.push_back(Block::new(5, 4));
        body.push_back(Block::new(4, 4));

        Snake {
            body,
            last_removed_block: None,
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d) {
        for block in self.body.iter() {
            draw_block(block.x, block.y, SNAKE_COLOR, context.transform, graphics);
        }
    }

    pub fn update(&mut self, direction: Direction) {
        let new_block: Block = self.next_head_position(direction);
        self.body.push_front(new_block);
        self.last_removed_block = self.body.pop_back();
    }

    pub fn head(&mut self) -> (u32, u32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn eat(&mut self) {
        match self.last_removed_block {
            Some(block) => self.body.push_back(block.clone()),
            None => {}
        }
    }

    pub fn check_alive(&mut self, direction: Direction, width: u32, height: u32) -> bool {
        let (head_x, head_y) = self.head();
        // 下一个位置会出现负数，如果调用 next_head_position 会使得程序 panic
        // 提前判断下这种情况，可以考虑加一个 border 来解决
        if (head_x == 0 && direction == Direction::Left)
            || (head_y == 0 && direction == Direction::Up)
        {
            return false;
        }

        let Block { x, y } = self.next_head_position(direction);
        // println!("{}, {}", x, y);
        !self.is_overlap_except_tail(x, y) && x < width && y < height
    }

    pub fn next_head_position(&mut self, direction: Direction) -> Block {
        let (head_x, head_y) = self.head();

        match direction {
            Direction::Up => Block::new(head_x, head_y - 1),
            Direction::Down => Block::new(head_x, head_y + 1),
            Direction::Left => Block::new(head_x - 1, head_y),
            Direction::Right => Block::new(head_x + 1, head_y),
        }
    }

    pub fn is_overlap_except_tail(&mut self, x: u32, y: u32) -> bool {
        let mut count = 0;

        for block in self.body.iter() {
            if count > self.body.len() - 1 {
                break;
            }
            if block.x == x && block.y == y {
                return true;
            }
            count += 1;
        }
        return false;
    }
}
