use std::collections::VecDeque;

#[derive(Copy, Clone, RustcDecodable)]
pub struct Pos {
    pub x: u8,
    pub y: u8
}

impl Pos {
    pub fn go(&self, dir: &Dir) -> Pos {
        match dir {
            &Dir::Up    => Pos { x: self.x, y: self.y - 1},
            &Dir::Down  => Pos { x: self.x, y: self.y + 1},
            &Dir::Left  => Pos { x: self.x - 1, y: self.y },
            &Dir::Right => Pos { x: self.x + 1, y: self.y }
        }
    }
}

#[derive(Copy, Clone, RustcDecodable)]
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

#[derive(RustcDecodable)]
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

        //println("{:?}", body_positions.size());
        body_positions
    }
}

#[derive(RustcDecodable)]
pub struct GameState {
    pub snakes: Vec<Snake>
}

impl GameState {
    pub fn new() -> GameState {
        let mut moves = VecDeque::new();
//        moves.push_front(Dir::Left);
//        moves.push_front(Dir::Left);
//        moves.push_front(Dir::Left);
//        moves.push_front(Dir::Left);

        let snakes = vec![
                            Snake {
                                head: Pos { x: 20, y: 20 },
                                moves: moves,
                                color: [1.0, 0.0, 1.0, 1.0]
                            }
                         ];

        GameState {
            snakes: snakes
        }
    }
}
