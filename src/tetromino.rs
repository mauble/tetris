use crate::global::*;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub id: usize,
    pub rotation: usize, // 0 = spawn, 1 = right, 2 = reverse, 3 = left
    pub pos: Vec2,
    pub color: (u8, u8, u8),
}

impl Tetromino {
    pub fn new(id: usize, pos: Option<Vec2>) -> Self {
        Tetromino {
            id,
            rotation: 0,
            pos: pos.unwrap_or(TETROMINO_SPAWN_POS),
            color: SHAPES[id].color,
        }
    }

    pub fn shape(&self) -> [[bool; 4]; 4] {
        SHAPES[self.id].orientations[self.rotation]
    }

    pub fn rotate(&mut self, clockwise: bool) {
        if clockwise {
            self.rotation = (self.rotation + 1) % 4;
        } else {
            self.rotation = (self.rotation + 3) % 4;
        }
    }

    pub fn center_offset(&self) -> f32 {
        let shape = self.shape();
        let count = (1..4)
            .flat_map(|row| (0..4).filter(move |&col| shape[row][col] && !shape[row - 1][col]))
            .count();

        if count == 3 {
            0.5
        } else {
            0.0
        }
    }
}
