use crate::global::*;
use macroquad::prelude::*;

pub struct Template {
    pub shape: [[bool; 4]; 4],
    pub color: (u8, u8, u8),
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub shape: [[bool; 4]; 4],
    pub color: (u8, u8, u8),
    pub pos: Vec2,
    pub phantom: bool,
    pub preview: bool,
}

impl Tetromino {
    pub fn new(id: usize, pos: Option<Vec2>, phantom: Option<bool>, center: Option<bool>) -> Self {
        Tetromino {
            shape: SHAPES[id].shape,
            color: SHAPES[id].color,
            pos: pos.unwrap_or(TETROMINO_SPAWN_POS),
            phantom: phantom.unwrap_or(false),
            preview: center.unwrap_or(false),
        }
    }

    pub fn rotate_shape(&mut self) {
        let mut new_shape: [[bool; 4]; 4] = [[false; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                new_shape[j][4 - i - 1] = self.shape[i][j];
            }
        }

        self.shape = new_shape;
    }

    pub fn preview_offset(&self) -> f32 {
        if self.preview {
            let mut count = 0;

            for row in 1..self.shape.len() {
                for col in 0..self.shape[row].len() {
                    if self.shape[row][col] && !self.shape[row - 1][col] {
                        count += 1;
                    }
                }
            }

            if count == 3 {
                return 0.5;
            }
        }
        return 0.0;
    }
}
