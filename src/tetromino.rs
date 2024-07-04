use crate::global::*;
use macroquad::prelude::*;

pub struct Template {
    pub width: u8,
    pub shape: [[bool; 4]; 4],
    pub color: (u8, u8, u8),
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub width: u8,
    pub shape: [[bool; 4]; 4],
    pub color: (u8, u8, u8),
    pub pos: Vec2,
}

impl Tetromino {
    pub fn new(id: usize, pos: Option<Vec2>) -> Self {
        Tetromino {
            pos: pos.unwrap_or(DEFAULT_TETROMINO_POS),
            width: SHAPES[id].width,
            shape: SHAPES[id].shape,
            color: SHAPES[id].color,
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
