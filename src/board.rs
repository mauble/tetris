use crate::global::*;
use crate::tetromino::*;
use macroquad::prelude::*;
use rand::ChooseRandom;

pub struct Board {
    pub font: Font,
    pub controls: Vec<KeyCode>,
    pub offset: u32,

    pub board: Vec<Vec<(u8, u8, u8)>>,
    pub bag: Vec<Tetromino>,
    pub direction: Vec2,

    pub t1: Tetromino,
    pub t2: Tetromino,
    pub p1: Tetromino,

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
    pub fn new(font: &Font, controls: Vec<KeyCode>, offset: u32) -> Self {
        let mut board = Board {
            font: font.clone(),
            controls,
            offset: offset * GAME_WIDTH as u32,

            board: vec![vec![BOARD_COLOR; BOARD_WIDTH]; BOARD_HEIGHT],
            bag: Vec::new(),
            direction: Vec2::new(0.0, 0.0),

            t1: Tetromino::new(0, None),
            t2: Tetromino::new(0, None),
            p1: Tetromino::new(0, None),

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
        board
    }

    pub fn run(&mut self) {
        self.input();
        self.update_phantom();
        self.draw();
    }

    fn update_bag(&mut self) {
        if self.bag.is_empty() {
            self.bag = SHAPES
                .iter()
                .enumerate()
                .map(|index| Tetromino::new(index.0, None))
                .collect();
            self.bag.shuffle();
        }
        self.t1 = self.t2;
        self.t1.pos = DEFAULT_TETROMINO_POS;
        self.t2 = self.bag.pop().unwrap();
        self.t2.pos = NEXT_TETROMINO_POS;
    }

    fn input(&mut self) {
        let time = get_current_time() as f64;
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
            let old = self.t1.shape;
            self.rotate_tetromino();
            if old != self.t1.shape && self.check_collision(self.t1, Some(Vec2::new(0.0, 1.0))) {
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

        if !self.check_collision(self.t1, Some(Vec2::new(self.direction.x, 0.0))) {
            self.t1.pos.x += self.direction.x;
        }
        if !self.check_collision(self.t1, Some(Vec2::new(0.0, self.direction.y))) {
            self.t1.pos.y += self.direction.y;
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
        false
    }

    fn update_phantom(&mut self) {
        self.p1 = self.t1.clone();
        for _ in 0..BOARD_HEIGHT + 1 {
            if !self.check_collision(self.p1, Some(Vec2::new(0.0, 1.0))) {
                self.p1.pos.y += 1.0;
            }
        }
    }

    fn drop_tetromino(&mut self) {
        self.t1 = self.p1;
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
        for x in 0..4 {
            if self.board[0][x] != BOARD_COLOR {
                self.t1 = Tetromino::new(0, None);
                self.t2 = Tetromino::new(0, None);
                self.p1 = Tetromino::new(0, None);

                self.bag.clear();
                self.board = vec![vec![BOARD_COLOR; BOARD_WIDTH]; BOARD_HEIGHT];
                self.direction = Vec2::new(0.0, 0.0);

                self.level = 0;
                self.score = 0;
                self.lines = 0;

                self.horizontal_delay = 120.0;
                self.drop_delay = 40.0;
                self.gravity_delay = 1000.0;

                self.last_horizontal = 0.0;
                self.last_drop = 0.0;
                self.last_gravity = 0.0;

                self.update_bag();
                return;
            }
        }
    }

    fn place_tetromino(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.t1.shape[y][x] {
                    let board_x = (self.t1.pos.x as i32 + x as i32) as usize;
                    let board_y = (self.t1.pos.y as i32 + y as i32) as usize;
                    if board_y < BOARD_HEIGHT {
                        self.board[board_y][board_x] = self.t1.color;
                    }
                }
            }
        }
        self.clear_lines();
        self.update_bag();
        self.check_game_over();
    }

    fn rotate_tetromino(&mut self) {
        let old = self.t1.clone();
        self.t1.rotate();

        for offset in [0, -1, 1, -2, 2] {
            self.t1.pos.x = old.pos.x + offset as f32;
            if !self.check_collision(self.t1, None) {
                return;
            }
        }

        self.t1 = old;
    }

    fn draw(&mut self) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                draw_block(
                    x as f32 + self.offset as f32,
                    y as f32,
                    self.board[y][x],
                    false,
                );
            }
        }

        self.p1.draw(self.offset, true, false);
        self.t1.draw(self.offset, false, false);
        self.t2.draw(self.offset, false, true);

        draw_ui_text(&self.font, "Score".to_string(), self.offset, 0.0);
        draw_ui_text(&self.font, self.score.to_string(), self.offset, 1.5);
        draw_ui_text(&self.font, "Lines".to_string(), self.offset, 4.5);
        draw_ui_text(&self.font, self.lines.to_string(), self.offset, 6.0);
        draw_ui_text(&self.font, "Level".to_string(), self.offset, 9.0);
        draw_ui_text(&self.font, self.level.to_string(), self.offset, 10.5);
    }
}
