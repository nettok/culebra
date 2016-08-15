extern crate piston_window;

use piston_window::*;

mod game;
use game::*;

impl Snake {
    fn draw<G>(&self, transform: math::Matrix2d, g: &mut G) where G: Graphics {
        for rect in self.to_body_rectangles() {
            rectangle(self.color, rect, transform, g);
        }
    }

    fn to_body_rectangles(&self) -> Vec<[f64; 4]> {
        self.to_body_positions()
            .iter()
            .map(|pos| Snake::body_rect(pos, 25.0))
            .collect::<Vec<[f64; 4]>>()
    }

    fn body_rect(pos: &Pos, size: f64) -> [f64; 4]  {
        [(pos.x as f64) * size, (pos.y as f64) * size, size, size]
    }
}

impl Game {
    fn render<G>(&self, args: &RenderArgs, c: Context, g: &mut G) where G: Graphics {
        clear([0.0; 4], g);
        self.snake.draw(c.transform, g);
    }

    fn key_press(&mut self, key: Key) {
        match key {
            Key::Up => self.snake.go(Dir::Up),
            Key::Down => self.snake.go(Dir::Down),
            Key::Left => self.snake.go(Dir::Left),
            Key::Right => self.snake.go(Dir::Right),
            _ => ()
        }
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Culebra!", [1000, 1000])
            .exit_on_esc(true).build().unwrap();

    let mut game = Game::new();

    let mut events = window.events();

    while let Some(e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                game.render(args, c, g);
            });
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_press(key);
        }
    }
}
