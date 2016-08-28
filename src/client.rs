extern crate piston_window;

use std::io;
use std::net::UdpSocket;

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

impl GameState {
    fn render<G>(&self, args: &RenderArgs, c: Context, g: &mut G) where G: Graphics {
        clear([0.0; 4], g);

        for snake in &self.snakes {
            snake.draw(c.transform, g);
        }
    }

    fn key_press(&mut self, key: Key) {
        match key {
            Key::Up => self.snakes[0].go(Dir::Up),
            Key::Down => self.snakes[0].go(Dir::Down),
            Key::Left => self.snakes[0].go(Dir::Left),
            Key::Right => self.snakes[0].go(Dir::Right),
            _ => ()
        }
    }
}

fn main() {
    // Start communication with game server

    let mut socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket.set_nonblocking(true).unwrap();
    socket.connect("127.0.0.1:7777").unwrap();
    socket.send("hola".as_bytes()).unwrap();

    // Graphics loop

    let mut window: PistonWindow =
        WindowSettings::new("Culebra!", [1000, 1000])
            .exit_on_esc(true).build().unwrap();

    let mut gs = GameState::new();

    let mut events = window.events();

    while let Some(e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                gs.render(args, c, g);
            });
        }

        if let Some(u) = e.update_args() {
            if let Some(new_game_state) = receive_game_state_from_server(&socket) {
                gs = new_game_state;
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            gs.key_press(key);
        }
    }
}

fn receive_game_state_from_server(socket: &UdpSocket) -> Option<GameState> {
    let mut recv_buf: [u8; 4096] = [0; 4096];

    match socket.recv(&mut recv_buf) {
        Ok(bytes_read) => {
            println!("received {:?} bytes", bytes_read);
            Some(GameState::new()) // TODO: deserialize message to GameState
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            // expected: client has not received a new message from the server
            None
        }
        Err(ref e) => {
            println!("error: kind={:?} {}", e.kind(), e);
            None
        }
    }
}
