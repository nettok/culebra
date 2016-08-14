extern crate piston_window;

use piston_window::*;

#[derive(Copy, Clone)]
struct Pos {
    x: u8,
    y: u8
}

impl Pos {
    fn up(&self) -> Pos {
        Pos { x: self.x, y: self.y - 1}
    }

    fn down(&self) -> Pos {
        Pos { x: self.x, y: self.y + 1}
    }

    fn left(&self) -> Pos {
        Pos { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Pos {
        Pos { x: self.x + 1, y: self.y }
    }

    fn go(&self, dir: &Dir) -> Pos {
        match dir {
            &Dir::Up    => self.up(),
            &Dir::Down  => self.down(),
            &Dir::Left  => self.left(),
            &Dir::Right => self.right()
        }
    }
}

enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    fn inverse(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Down,
            &Dir::Down  => Dir::Up,
            &Dir::Left  => Dir::Right,
            &Dir::Right => Dir::Left
        }
    }
}

struct Snake {
    head:  Pos,
    moves: Vec<Dir>,
    color: types::Color,
}

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

    fn to_body_positions(&self) -> Vec<Pos> {
        let mut body_positions = vec![self.head];

        let mut curr_pos = self.head;

        for mov in &self.moves {
            curr_pos = curr_pos.go(&mov.inverse());
            body_positions.push(curr_pos);
        }

        body_positions
    }

    fn body_rect(pos: &Pos, size: f64) -> [f64; 4]  {
        [(pos.x as f64) * size, (pos.y as f64) * size, size, size]
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Culebra!", [1000, 1000])
            .exit_on_esc(true).build().unwrap();

    let snake = Snake {
        head: Pos { x: 20, y: 20 },
        moves: vec![Dir::Up, Dir::Up, Dir::Left, Dir::Left, Dir::Left, Dir::Left],
        color: [1.0, 0.0, 1.0, 1.0]
    };

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);

            snake.draw(c.transform, g);
        });
    }
}
