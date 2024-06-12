use crate::global::*;
use crate::tetromino::*;
use macroquad::prelude::*;

pub struct Board {
    pub board: Vec<Vec<(u8, u8, u8)>>,
    pub tetromino: Tetromino,
    pub controls: Vec<KeyCode>,
    pub offset: u32,
    pub direction: Vec2,
}

impl Board {
    pub fn new(
        board: Vec<Vec<(u8, u8, u8)>>,
        tetromino: Tetromino,
        controls: Vec<KeyCode>,
        offset: u32,
        direction: Option<Vec2>,
    ) -> Self {
        Board {
            board,
            tetromino,
            controls,
            offset: offset * GAME_WIDTH as u32,
            direction: direction.unwrap_or(Vec2::new(0.0, 0.0)),
        }
    }

    pub fn run(&mut self) {
        self.input();
        self.check_game_end();
        self.draw();
    }

    fn input(&mut self) {
        self.direction = Vec2::new(0.0, 0.0);

        if is_key_down(self.controls[0]) {
            self.direction.x = -1.0;
        }
        if is_key_down(self.controls[1]) {
            self.direction.x = 1.0;
        }
        if is_key_down(self.controls[2]) {
            self.direction.y = 1.0;
        }
        if is_key_pressed(self.controls[3]) {
            self.rotate_tetromino();
        }
        if is_key_pressed(self.controls[4]) {
            self.drop_tetromino();
        }

        self.tetromino.pos.x += self.direction.x;
        if self.check_collision(self.tetromino.pos) {
            self.tetromino.pos.x -= self.direction.x;
        }
        self.tetromino.pos.y += self.direction.y;
        if self.check_collision(self.tetromino.pos) {
            self.tetromino.pos.y -= self.direction.y;
        }
    }

    fn check_game_end(&mut self) {
        if self.check_collision(self.tetromino.pos + Vec2::new(0.0, 1.0)) {
            self.place_tetromino();
            self.tetromino = new_tetromino();
        }
    }

    fn check_collision(&mut self, pos: Vec2) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.tetromino.shape[y][x] {
                    let new_x = pos.x as i32 + x as i32;
                    let new_y = pos.y as i32 + y as i32;

                    if new_x < 0 || new_x >= BOARD_WIDTH as i32 || new_y >= BOARD_HEIGHT as i32 {
                        return true;
                    }
                    if new_y > 0 && self.board[new_y as usize][new_x as usize] != (0, 0, 0) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn drop_tetromino(&mut self) {
        for _ in 0..BOARD_HEIGHT + 1 {
            if !self.check_collision(self.tetromino.pos) {
                self.tetromino.pos.y += 1.0;
            }
        }
        self.tetromino.pos.y -= 1.0;
        self.place_tetromino();
    }

    fn place_tetromino(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.tetromino.shape[y][x] {
                    let board_x = (self.tetromino.pos.x as i32 + x as i32) as usize;
                    let board_y = (self.tetromino.pos.y as i32 + y as i32) as usize;
                    if board_y < BOARD_HEIGHT {
                        self.board[board_y][board_x] = self.tetromino.color;
                    }
                }
            }
        }
    }

    fn rotate_tetromino(&mut self) {
        let old = self.tetromino.clone();
        self.tetromino.rotate();

        let left = self.check_collision(self.tetromino.pos + Vec2::new(-2.0, 0.0));
        let right = self.check_collision(self.tetromino.pos + Vec2::new(2.0, 0.0));

        if left && right {
            self.tetromino = old;
        }
        if right {
            while self.check_collision(self.tetromino.pos) {
                self.tetromino.pos.x -= 1.0;
            }
        }
        if left {
            while self.check_collision(self.tetromino.pos) {
                self.tetromino.pos.x += 1.0;
            }
        }
    }

    fn draw(&mut self) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.board[y][x] != (0, 0, 0) {
                    draw_block(x as f32 + self.offset as f32, y as f32, self.board[y][x]);
                }
            }
        }

        self.tetromino.draw(self.offset);
    }
}
