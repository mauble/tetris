use crate::global::*;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub width: u8,
    pub shape: [[bool; 4]; 4],
    pub color: (u8, u8, u8),
    pub pos: Vec2,
}

impl Tetromino {
    pub fn new(width: u8, shape: [[bool; 4]; 4], color: (u8, u8, u8), pos: Option<Vec2>) -> Self {
        Tetromino {
            width,
            shape,
            color,
            pos: pos.unwrap_or(DEFAULT_TETROMINO_POS),
        }
    }

    pub fn rotate(&mut self) {
        let mut new_shape: [[bool; 4]; 4] = [[false; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                new_shape[j][4 - i - 1] = self.shape[i][j];
            }
        }

        self.shape = new_shape;
    }

    pub fn draw(&mut self, offset: u32, phantom: bool, center: bool) {
        let next_tetromino_offset = if center {
            (4.0 - self.width as f32) / 2.0
        } else {
            0.0
        };
        for y in 0..4 {
            for x in 0..4 {
                if self.shape[y][x] {
                    draw_block(
                        self.pos.x + x as f32 - next_tetromino_offset + offset as f32,
                        self.pos.y + y as f32,
                        self.color,
                        phantom,
                    );
                }
            }
        }
    }
}
