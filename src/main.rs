extern crate piston_window;

mod drawing;
mod game;
mod snake;

use game::Game;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snake", [500, 500])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(args) = event.update_args() {
            game.update(args.dt);
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            game.draw(context, graphics);
        });
    }
}
