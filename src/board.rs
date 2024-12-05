use crate::global::*;
use crate::tetromino::*;
use macroquad::prelude::*;
use rand::ChooseRandom;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Board {
    pub controls: Vec<KeyCode>,
    pub offset: u32,

    pub board: Vec<Vec<(u8, u8, u8)>>,
    pub bag: Vec<usize>,
    pub direction: Vec2,

    pub piece: Tetromino,
    pub preview: Tetromino,
    pub phantom: Tetromino,

    pub horizontal_delay: f64,
    pub drop_delay: f64,
    pub gravity_delay: f64,

    pub last_horizontal: f64,
    pub last_drop: f64,
    pub last_gravity: f64,

    pub level: i32,
    pub score: i32,
    pub lines: i32,
}

impl Board {
    pub fn new(controls: Vec<KeyCode>, offset: u32) -> Self {
        let mut board = Board {
            controls,
            offset: offset * GAME_WIDTH as u32,

            board: vec![vec![BOARD_COLOR; BOARD_WIDTH]; BOARD_HEIGHT],
            bag: Vec::new(),
            direction: Vec2::new(0.0, 0.0),

            piece: Tetromino::new(0, None, None, None),
            phantom: Tetromino::new(0, None, None, None),
            preview: Tetromino::new(0, None, None, None),

            horizontal_delay: 120.0,
            drop_delay: 40.0,
            gravity_delay: 1000.0,

            last_horizontal: 0.0,
            last_drop: 0.0,
            last_gravity: 0.0,

            level: 0,
            score: 0,
            lines: 0,
        };

        board.update_bag();
        return board;
    }

    pub fn run(&mut self) {
        self.input();
        self.update_phantom();
    }

    fn update_bag(&mut self) {
        if self.bag.is_empty() {
            self.bag = SHAPES.iter().enumerate().map(|(index, _)| index).collect();
            self.bag.shuffle();
        }

        self.piece = self.preview.clone();
        self.piece.center = false;
        self.piece.pos = DEFAULT_TETROMINO_POS;

        self.preview = Tetromino::new(self.bag.pop().unwrap(), None, None, Some(true));
        self.preview.pos = NEXT_TETROMINO_POS;
    }

    fn input(&mut self) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64;
        self.direction = Vec2::new(0.0, 0.0);

        if time - self.last_horizontal >= self.horizontal_delay {
            if is_key_down(self.controls[0]) {
                self.direction.x = -1.0;
                self.last_horizontal = time;
            }
            if is_key_down(self.controls[1]) {
                self.direction.x = 1.0;
                self.last_horizontal = time;
            }
        }

        if time - self.last_drop >= self.drop_delay {
            if is_key_down(self.controls[2]) {
                self.direction.y = 1.0;
                self.last_drop = time;
            }
        }

        if is_key_pressed(self.controls[3]) {
            let old = self.piece.shape;
            self.rotate_tetromino();
            if old != self.piece.shape
                && self.check_collision(self.piece, Some(Vec2::new(0.0, 1.0)))
            {
                self.last_gravity = time;
            }
        }
        if is_key_pressed(self.controls[4]) {
            self.drop_tetromino();
        }

        if time - self.last_gravity >= self.gravity_delay {
            self.direction.y = 1.0;
            self.last_gravity = time;
        }

        if !self.check_collision(self.piece, Some(Vec2::new(self.direction.x, 0.0))) {
            self.piece.pos.x += self.direction.x;
        }
        if !self.check_collision(self.piece, Some(Vec2::new(0.0, self.direction.y))) {
            self.piece.pos.y += self.direction.y;
        } else if self.direction.y == 1.0 {
            self.place_tetromino();
        }
    }

    fn check_collision(&mut self, tetromino: Tetromino, offset: Option<Vec2>) -> bool {
        let offset = offset.unwrap_or(Vec2::new(0.0, 0.0));
        for y in 0..4 {
            for x in 0..4 {
                if tetromino.shape[y][x] {
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
        self.phantom.phantom = true;
        for _ in 0..BOARD_HEIGHT + 1 {
            if !self.check_collision(self.phantom, Some(Vec2::new(0.0, 1.0))) {
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
        self.lines += new_lines as i32;
        self.score +=
            vec![0, 40, 100, 300, 1200][new_lines] * (self.level + 1) + 2 * (self.level + 1);
        if self.level != self.lines / 10 {
            self.level = self.lines / 10;
            self.gravity_delay = 1000 as f64 / (self.level + 1) as f64 + 200.0;
        }
    }

    fn check_game_over(&mut self) {
        for x in 0..BOARD_WIDTH {
            if self.board[0][x] != BOARD_COLOR {
                *self = Board::new(self.controls.clone(), self.offset);
                return;
            }
        }
    }

    fn place_tetromino(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.piece.shape[y][x] {
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

    fn rotate_tetromino(&mut self) {
        let old = self.piece.clone();
        self.piece.rotate();

        for offset in [0, -1, 1, -2, 2] {
            self.piece.pos.x = old.pos.x + offset as f32;
            if !self.check_collision(self.piece, None) {
                return;
            }
        }

        self.piece = old;
    }
}
