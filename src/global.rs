use crate::tetromino::*;
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub const BLOCK_SIZE: f32 = 40.0;
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const GAME_WIDTH: usize = BOARD_WIDTH + 5;

pub const DEFAULT_TETROMINO_POS: Vec2 = Vec2::new((BOARD_WIDTH / 2 - 2) as f32, -1.0);
pub const NEXT_TETROMINO_POS: Vec2 = Vec2::new(BOARD_WIDTH as f32 + 0.5, 1.0);

pub const BOARD_COLOR: (u8, u8, u8) = (20, 20, 20);
pub const UI_COLOR: (u8, u8, u8) = (30, 30, 30);
pub const FONT_COLOR: (u8, u8, u8) = (255, 255, 255);

pub const FONT_SIZE: u16 = 36;

pub const FONT_PATH: &'static str = "./assets/font.ttf";
// pub const ICON_PATH: &'static str = "./assets/icon.ttf";

pub const SHAPES: [TetrominoTemplate; 7] = [
    TetrominoTemplate {
        width: 4,
        shape: [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ],
        color: (0, 190, 225),
    },
    TetrominoTemplate {
        width: 3,
        shape: [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (60, 60, 230),
    },
    TetrominoTemplate {
        width: 3,
        shape: [
            [false, false, false, false],
            [false, false, false, true],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (220, 150, 50),
    },
    TetrominoTemplate {
        width: 4,
        shape: [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        color: (240, 215, 0),
    },
    TetrominoTemplate {
        width: 3,
        shape: [
            [false, false, false, false],
            [false, false, true, true],
            [false, true, true, false],
            [false, false, false, false],
        ],
        color: (135, 220, 130),
    },
    TetrominoTemplate {
        width: 3,
        shape: [
            [false, false, false, false],
            [false, false, true, false],
            [false, true, true, true],
            [false, false, false, false],
        ],
        color: (110, 40, 230),
    },
    TetrominoTemplate {
        width: 3,
        shape: [
            [false, false, false, false],
            [false, true, true, false],
            [false, false, true, true],
            [false, false, false, false],
        ],
        color: (220, 60, 90),
    },
];

pub fn draw_ui_text(font: &Font, text: String, x_offset: u32, y: f32) {
    let center = get_text_center(&text, Some(&font), FONT_SIZE, 1.0, 0.0);
    draw_text_ex(
        &text,
        (BOARD_WIDTH as f32 + x_offset as f32 + 2.5) * BLOCK_SIZE - center.x,
        (y + NEXT_TETROMINO_POS.y + 6.5) * BLOCK_SIZE,
        TextParams {
            font_size: FONT_SIZE,
            font: Some(&font),
            color: get_color(FONT_COLOR, 255),
            ..Default::default()
        },
    );
}

pub fn get_current_time() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn get_color(color: (u8, u8, u8), a: u8) -> Color {
    return Color::from_rgba(color.0, color.1, color.2, a);
}

pub fn draw_block(x: f32, y: f32, color: (u8, u8, u8), phantom: bool) {
    let x_loc = x * BLOCK_SIZE;
    let y_loc = y * BLOCK_SIZE;

    let color_var = Color::from_rgba(color.0, color.1, color.2, 255);

    if phantom {
        draw_rectangle_lines(
            x_loc + 2.0,
            y_loc + 2.0,
            BLOCK_SIZE - 5.0,
            BLOCK_SIZE - 5.0,
            2.0,
            color_var,
        );
    } else {
        draw_rectangle(
            x_loc + 1.0,
            y_loc + 1.0,
            BLOCK_SIZE - 2.0,
            BLOCK_SIZE - 2.0,
            Color::from_rgba(
                (color.0 as f32 / 1.2) as u8,
                (color.1 as f32 / 1.2) as u8,
                (color.2 as f32 / 1.2) as u8,
                150,
            ),
        );
        draw_rectangle_lines(
            x_loc,
            y_loc,
            BLOCK_SIZE,
            BLOCK_SIZE,
            1.0,
            Color::from_rgba(UI_COLOR.0, UI_COLOR.1, UI_COLOR.2, 255),
        );
        draw_rectangle(
            x_loc + 4.0,
            y_loc + 4.0,
            BLOCK_SIZE - 8.0,
            BLOCK_SIZE - 8.0,
            color_var,
        );
    }
}
