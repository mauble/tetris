use crate::tetromino::*;
use macroquad::prelude::*;

pub const BOARD_COLOR: (u8, u8, u8) = (20, 20, 20);
pub const UI_COLOR: (u8, u8, u8) = (30, 30, 30);

pub const FONT_COLOR: (u8, u8, u8) = (255, 255, 255);
pub const FONT_SIZE: u16 = 36;
pub const FONT_PATH: &str = "./assets/font.ttf";

pub const BLOCK_SIZE: f32 = 40.0;
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const GAME_WIDTH: usize = BOARD_WIDTH + 5;

pub const DEFAULT_TETROMINO_POS: Vec2 = Vec2::new((BOARD_WIDTH / 2 - 2) as f32, -1.0);
pub const NEXT_TETROMINO_POS: Vec2 = Vec2::new(BOARD_WIDTH as f32 + 0.5, 1.0);

pub const SHAPES: [Template; 7] = [
    Template {
        shape: [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ],
        color: (0, 190, 225),
    },
    Template {
        shape: [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (60, 60, 230),
    },
    Template {
        shape: [
            [false, false, false, false],
            [false, false, false, true],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (220, 150, 50),
    },
    Template {
        shape: [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        color: (240, 215, 0),
    },
    Template {
        shape: [
            [false, false, false, false],
            [false, false, true, true],
            [false, true, true, false],
            [false, false, false, false],
        ],
        color: (135, 220, 130),
    },
    Template {
        shape: [
            [false, false, false, false],
            [false, false, true, false],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (110, 40, 230),
    },
    Template {
        shape: [
            [false, false, false, false],
            [false, true, true, false],
            [false, false, true, true],
            [false, false, false, false],
        ],
        color: (220, 60, 90),
    },
];

pub fn get_color(color: (u8, u8, u8), a: u8) -> Color {
    return Color::from_rgba(color.0, color.1, color.2, a);
}
