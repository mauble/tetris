use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_millis() -> u64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
}

pub fn get_color(color: (u8, u8, u8), a: u8) -> Color {
    return Color::from_rgba(color.0, color.1, color.2, a);
}

pub fn get_pos(grid_pos: Vec2) -> Vec2 {
    return grid_pos * BLOCK_SIZE;
}

pub const FONT_COLOR: (u8, u8, u8) = (255, 255, 255);
pub const BOARD_COLOR: (u8, u8, u8) = (20, 20, 20);
pub const UI_COLOR: (u8, u8, u8) = (30, 30, 30);

pub const FONT_SIZE: u16 = 32;
pub const BLOCK_SIZE: f32 = 40.0;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub const GAME_WIDTH: usize = BOARD_WIDTH + 5;

pub const X_MOVE_DELAY: u64 = 120;
pub const Y_MOVE_DELAY: u64 = 50;
pub const GRAVITY_DELAY: u64 = 1000;
pub const LOCK_DELAY: u64 = 200;

pub const TETROMINO_SPAWN_POS: Vec2 = Vec2::new((BOARD_WIDTH / 2 - 2) as f32, -2.0);
pub const TETROMINO_PREVIEW_POS: Vec2 = Vec2::new(BOARD_WIDTH as f32 + 0.5, 2.0);

pub const SCORE_PER_LINE: [u32; 5] = [2, 100, 300, 500, 800];

pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,
    pub rotate_clockwise: KeyCode,
    pub rotate_counterclockwise: KeyCode,
    pub pause: KeyCode,
}

pub const CONTROLS_PLAYER1: Controls = Controls {
    left: KeyCode::A,
    right: KeyCode::D,
    soft_drop: KeyCode::S,
    hard_drop: KeyCode::Space,
    rotate_clockwise: KeyCode::W,
    rotate_counterclockwise: KeyCode::LeftShift,
    pause: KeyCode::Escape,
};

pub const JLSTZ_KICKS: [((i32, i32), [(i32, i32); 5]); 8] = [
    ((0, 1), [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]),
    ((1, 0), [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]),
    ((1, 2), [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]),
    ((2, 1), [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]),
    ((2, 3), [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]),
    ((3, 2), [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]),
    ((3, 0), [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]),
    ((0, 3), [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]),
];

pub const I_KICKS: [((i32, i32), [(i32, i32); 5]); 8] = [
    ((0, 1), [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]),
    ((1, 0), [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]),
    ((1, 2), [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]),
    ((2, 1), [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]),
    ((2, 3), [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]),
    ((3, 2), [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]),
    ((3, 0), [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]),
    ((0, 3), [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]),
];

pub struct Template {
    pub orientations: [[[bool; 4]; 4]; 4],
    pub color: (u8, u8, u8),
}

pub const SHAPES: [Template; 7] = [
    // I
    Template {
        orientations: [
            [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        ],
        color: (0, 190, 225),
    },
    // J
    Template {
        orientations: [
            [
                [false, false, false, false],
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [true, true, false, false],
            ],
        ],
        color: (60, 60, 230),
    },
    // L
    Template {
        orientations: [
            [
                [false, false, false, false],
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        ],
        color: (220, 150, 50),
    },
    // O
    Template {
        orientations: [
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
        ],
        color: (240, 215, 0),
    },
    // S
    Template {
        orientations: [
            [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
            ],
            [
                [false, false, false, false],
                [true, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
            ],
        ],
        color: (135, 220, 130),
    },
    // T
    Template {
        orientations: [
            [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [false, true, false, false],
            ],
        ],
        color: (110, 40, 230),
    },
    // Z
    Template {
        orientations: [
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [true, false, false, false],
            ],
        ],
        color: (220, 60, 90),
    },
];
