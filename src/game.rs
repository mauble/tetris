use crate::global::*;
use crate::tetromino::Tetromino;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad::rand::RandGenerator;

pub struct Game<'a> {
    controls: &'a Controls,
    bag: Vec<usize>,

    pub board: Vec<Vec<(u8, u8, u8)>>,

    pub piece: Tetromino,
    pub preview: Tetromino,
    pub phantom: Tetromino,

    pub score: u32,
    pub lines: u32,
    pub level: u32,

    direction: Vec2,

    x_move_delay: u64,
    y_move_delay: u64,
    gravity_delay: u64,
    lock_delay: u64,

    last_x_move: u64,
    last_y_move: u64,
    last_gravity: u64,
    last_lock: u64,
}

impl<'a> Game<'a> {
    pub fn new(controls: &'a Controls) -> Self {
        let mut game = Game {
            controls,
            bag: Vec::new(),

            board: vec![vec![BOARD_COLOR; BOARD_WIDTH]; BOARD_HEIGHT],

            piece: Tetromino::new(0, TETROMINO_SPAWN_POS),
            phantom: Tetromino::new(0, TETROMINO_SPAWN_POS),
            preview: Tetromino::new(0, TETROMINO_PREVIEW_POS),

            score: 0,
            lines: 0,
            level: 0,

            direction: Vec2::new(0.0, 0.0),

            x_move_delay: X_MOVE_DELAY,
            y_move_delay: Y_MOVE_DELAY,
            gravity_delay: GRAVITY_DELAY,
            lock_delay: LOCK_DELAY,

            last_x_move: 0,
            last_y_move: 0,
            last_gravity: 0,
            last_lock: 0,
        };

        game.refill_bag();

        game.piece = Tetromino::new(game.bag.pop().unwrap(), TETROMINO_SPAWN_POS);
        game.preview = Tetromino::new(game.bag.pop().unwrap(), TETROMINO_PREVIEW_POS);

        return game;
    }

    pub fn update(&mut self) {
        self.input();
        self.update_phantom();
    }

    fn refill_bag(&mut self) {
        self.bag = (0..SHAPES.len()).collect();
        let mut rng = RandGenerator::new();
        rng.srand(get_millis());
        self.bag.shuffle_with_state(&mut rng);
    }

    fn update_bag(&mut self) {
        self.piece = self.preview.clone();
        self.piece.pos = TETROMINO_SPAWN_POS;

        if self.bag.is_empty() {
            self.refill_bag();
        }

        self.preview = Tetromino::new(self.bag.pop().unwrap(), TETROMINO_PREVIEW_POS);
    }

    fn input(&mut self) {
        let time = get_millis();
        self.direction = Vec2::new(0.0, 0.0);

        if time - self.last_x_move >= self.x_move_delay {
            if is_key_down(self.controls.left) {
                self.direction.x = -1.0;
                self.last_x_move = time;
            }

            if is_key_down(self.controls.right) {
                self.direction.x = 1.0;
                self.last_x_move = time;
            }
        }

        if time - self.last_y_move >= self.y_move_delay {
            if is_key_down(self.controls.soft_drop) {
                self.direction.y = 1.0;
                self.last_y_move = time;
            }
        }

        if is_key_pressed(self.controls.hard_drop) {
            self.drop_tetromino();
            return;
        }

        let mut clockwise = false;
        if is_key_pressed(self.controls.rotate_clockwise) {
            clockwise = true;
            self.rotate_tetromino(clockwise);
        } else if is_key_pressed(self.controls.rotate_counterclockwise) {
            self.rotate_tetromino(clockwise);
        }

        if time - self.last_gravity >= self.gravity_delay {
            self.direction.y = 1.0;
            self.last_gravity = time;
        }

        if !self.check_collision(self.piece, Vec2::new(self.direction.x, 0.0))
            && self.direction.x != 0.0
        {
            self.piece.pos.x += self.direction.x;
        }

        if !self.check_collision(self.piece, Vec2::new(0.0, self.direction.y))
            && self.direction.y != 0.0
        {
            self.piece.pos.y += self.direction.y;
            self.last_lock = 0;
        } else if self.direction.y > 0.0 {
            if self.last_lock == 0 {
                self.last_lock = time;
            } else if time - self.last_lock >= self.lock_delay {
                self.place_tetromino();
            }
        }
    }

    fn check_collision(&mut self, tetromino: Tetromino, offset: Vec2) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if tetromino.shape()[y][x] {
                    let mut index = Vec2::new(x as f32, y as f32);
                    index += tetromino.pos + offset;

                    if index.x < 0.0
                        || index.x >= BOARD_WIDTH as f32
                        || index.y >= BOARD_HEIGHT as f32
                    {
                        return true;
                    }
                    if index.y > 0.0
                        && self.board[index.y as usize][index.x as usize] != BOARD_COLOR
                    {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn update_phantom(&mut self) {
        self.phantom = self.piece.clone();
        for _ in 0..BOARD_HEIGHT + 1 {
            if !self.check_collision(self.phantom, Vec2::new(0.0, 1.0)) {
                self.phantom.pos.y += 1.0;
            }
        }
    }

    fn drop_tetromino(&mut self) {
        self.piece = self.phantom;
        self.place_tetromino();
    }

    fn clear_lines(&mut self) {
        let mut cleared_lines = Vec::new();

        for y in 0..BOARD_HEIGHT {
            if self.board[y].iter().all(|&cell| cell != BOARD_COLOR) {
                cleared_lines.push(y);
            }
        }

        for &line in cleared_lines.iter() {
            self.board.remove(line);
            self.board.insert(0, vec![BOARD_COLOR; BOARD_WIDTH]);
        }

        let new_lines = cleared_lines.len();
        self.lines += new_lines as u32;
        self.score += SCORE_PER_LINE[new_lines] * (self.level + 1);
        if self.level != self.lines / 10 {
            self.level = self.lines / 10;
            self.gravity_delay = (1000 / (self.level + 1) + 200) as u64;
        }
    }

    fn check_game_over(&mut self) {
        for x in 0..BOARD_WIDTH {
            if self.board[0][x] != BOARD_COLOR {
                *self = Game::new(&self.controls);
                return;
            }
        }
    }

    fn place_tetromino(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.piece.shape()[y][x] {
                    let board_x = (self.piece.pos.x as i32 + x as i32) as usize;
                    let board_y = (self.piece.pos.y as i32 + y as i32) as usize;
                    if board_y < BOARD_HEIGHT {
                        self.board[board_y][board_x] = self.piece.color;
                    }
                }
            }
        }

        self.clear_lines();
        self.update_bag();
        self.check_game_over();
    }

    fn rotate_tetromino(&mut self, clockwise: bool) {
        let from = self.piece.rotation as i32;
        let mut test_piece = self.piece.clone();
        test_piece.rotate(clockwise);
        let to = test_piece.rotation as i32;

        let kicks = match self.piece.id {
            0 => I_KICKS
                .iter()
                .find(|((f, t), _)| *f == from && *t == to)
                .map(|(_, kicks)| kicks),
            3 => Some(&[(0, 0); 5]),
            _ => JLSTZ_KICKS
                .iter()
                .find(|((f, t), _)| *f == from && *t == to)
                .map(|(_, kicks)| kicks),
        };

        if let Some(kicks) = kicks {
            for &(kx, ky) in kicks.iter() {
                let offset = Vec2::new(kx as f32, ky as f32);
                if !self.check_collision(test_piece.clone(), offset) {
                    self.piece.rotate(clockwise);
                    self.piece.pos += offset;
                    self.last_lock = get_millis();
                    return;
                }
            }
        }
    }
}
