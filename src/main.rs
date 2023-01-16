extern crate piston_window;

mod drawing;
mod game;
mod snake;

use drawing::BACKGROUND_COLOR;
use game::Game;
use piston_window::*;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;
const BLOCK_SIZE: u32 = 20;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Rust Snake", [WIDTH * BLOCK_SIZE, HEIGHT * BLOCK_SIZE])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(WIDTH, HEIGHT);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(args) = event.update_args() {
            game.update(args.dt);
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear(BACKGROUND_COLOR, graphics);
            game.draw(context, graphics);
        });
    }
}
