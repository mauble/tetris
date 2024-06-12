use crate::global::*;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub pos: Vec2,
    pub shape: [[bool; 4]; 4],
    pub color: (u8, u8, u8),
}

impl Tetromino {
    pub fn new(pos: Vec2, shape: [[bool; 4]; 4], color: (u8, u8, u8)) -> Self {
        Tetromino { pos, shape, color }
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

    pub fn draw(&mut self, offset: u32) {
        for y in 0..4 {
            for x in 0..4 {
                if self.shape[y][x] {
                    draw_block(
                        self.pos.x + x as f32 + offset as f32,
                        self.pos.y + y as f32,
                        self.color,
                    );
                }
            }
        }
    }
}
