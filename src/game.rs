use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: u8,
    pub y: u8
}

impl Pos {
    pub fn up(&self) -> Pos {
        Pos { x: self.x, y: self.y - 1}
    }

    pub fn down(&self) -> Pos {
        Pos { x: self.x, y: self.y + 1}
    }

    pub fn left(&self) -> Pos {
        Pos { x: self.x - 1, y: self.y }
    }

    pub fn right(&self) -> Pos {
        Pos { x: self.x + 1, y: self.y }
    }

    pub fn go(&self, dir: &Dir) -> Pos {
        match dir {
            &Dir::Up    => self.up(),
            &Dir::Down  => self.down(),
            &Dir::Left  => self.left(),
            &Dir::Right => self.right()
        }
    }
}

#[derive(Copy, Clone)]
pub enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    pub fn inverse(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Down,
            &Dir::Down  => Dir::Up,
            &Dir::Left  => Dir::Right,
            &Dir::Right => Dir::Left
        }
    }
}

pub struct Snake {
    pub head:  Pos,
    pub moves: VecDeque<Dir>,
    pub color: [f32; 4],
}

impl Snake {
    pub fn go(&mut self, dir: Dir) {
        self.head = self.head.go(&dir);
        self.moves.push_front(dir);
        self.moves.pop_back();
    }

    pub fn to_body_positions(&self) -> Vec<Pos> {
        let mut body_positions = vec![self.head];

        let mut curr_pos = self.head;

        for mov in &self.moves {
            curr_pos = curr_pos.go(&mov.inverse());
            body_positions.push(curr_pos);
        }

        body_positions
    }
}

pub struct Game {
    pub snake: Snake
}

impl Game {
    pub fn new() -> Game {
        let mut moves = VecDeque::new();
        moves.push_front(Dir::Left);
        moves.push_front(Dir::Left);
        moves.push_front(Dir::Left);
        moves.push_front(Dir::Left);

        Game {
            snake: Snake {
                head: Pos { x: 20, y: 20 },
                moves: moves,
                color: [1.0, 0.0, 1.0, 1.0]
            }
        }
    }
}
