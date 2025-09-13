use crate::assets::Assets;
use crate::game::Game;
use crate::global::*;
use crate::tetromino::Tetromino;
use macroquad::prelude::*;

pub struct Renderer<'a> {
    assets: &'a Assets,
    controls: &'a Controls,
    pub paused: bool,
}

impl<'a> Renderer<'a> {
    pub fn new(assets: &'a Assets, controls: &'a Controls) -> Self {
        Self {
            assets,
            controls,
            paused: false,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(self.controls.pause) {
            self.paused = !self.paused;
        }
    }

    pub fn draw(&self, game: &Game) {
        self.draw_left_panel(&game);
        self.draw_center_panel(&game);
        self.draw_right_panel(&game);
        self.draw_paused();
    }

    fn draw_left_panel(&self, game: &Game) {
        let x_text = GAME_SIDE_WIDTH / 2.0;
        let y_text = 2.0;

        self.draw_text("Hold:", FONT_SIZE, Vec2::new(x_text, y_text), true);

        if game.empty_hold {
            self.draw_text("Press C.", SMALL_FONT_SIZE, Vec2::new(x_text, 5.0), true);
        } else {
            self.draw_tetromino(game.hold, false, Vec2::new(x_text, 3.5));
        }
    }

    fn draw_center_panel(&self, game: &Game) {
        for y in 0..BOARD_HEIGHT as usize {
            for x in 0..BOARD_WIDTH as usize {
                Self::draw_block(
                    Vec2::new(x as f32 + GAME_SIDE_WIDTH, y as f32),
                    game.board[y][x],
                    false,
                );
            }
        }

        self.draw_tetromino(game.piece, false, Vec2::ZERO);
        self.draw_tetromino(game.phantom, true, Vec2::ZERO);
    }

    fn draw_right_panel(&self, game: &Game) {
        let x_text = GAME_SIDE_WIDTH + BOARD_WIDTH as f32 + GAME_SIDE_WIDTH / 2.0;
        let y_text = BOARD_HEIGHT as f32 - 2.0;

        self.draw_text("Next:", FONT_SIZE, Vec2::new(x_text, 2.0), true);
        self.draw_text("Score", FONT_SIZE, Vec2::new(x_text, y_text - 8.5), true);
        self.draw_text(
            &game.score.to_string(),
            FONT_SIZE,
            Vec2::new(x_text, y_text - 7.0),
            true,
        );
        self.draw_text("Lines", FONT_SIZE, Vec2::new(x_text, y_text - 5.0), true);
        self.draw_text(
            &game.lines.to_string(),
            FONT_SIZE,
            Vec2::new(x_text, y_text - 3.5),
            true,
        );
        self.draw_text("Level", FONT_SIZE, Vec2::new(x_text, y_text - 1.5), true);
        self.draw_text(
            &game.level.to_string(),
            FONT_SIZE,
            Vec2::new(x_text, y_text),
            true,
        );

        self.draw_tetromino(game.preview, false, Vec2::new(x_text, 3.5));
    }

    fn draw_paused(&self) {
        if self.paused {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.95),
            );

            let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.25);
            self.draw_text("Pause", FONT_SIZE, center, false);

            let controls_start = Vec2::new(screen_width() * 0.5, screen_height() * 0.45);
            let line_spacing = 50.0;

            let controls = [
                "↑ - Rotate Clockwise",
                "← / → - Move Left / Right",
                "↓ - Soft Drop",
                "Space - Hard Drop",
                "Ctrl - Rotate Counterclockwise",
                "C - Hold",
            ];

            for (i, &line) in controls.iter().enumerate() {
                let pos = controls_start + Vec2::new(0.0, i as f32 * line_spacing);
                self.draw_text(line, SMALL_FONT_SIZE, pos, false);
            }
        }
    }

    fn draw_text(&self, text: &str, font_size: u16, pos: Vec2, grid: bool) {
        let mut calc_pos = pos;
        if grid {
            calc_pos = get_pos(pos);
        }
        calc_pos -= get_text_center(&text, Some(&self.assets.font), font_size, 1.0, 0.0);
        draw_text_ex(
            &text,
            calc_pos.x,
            calc_pos.y,
            TextParams {
                font_size: font_size,
                font: Some(&self.assets.font),
                color: get_color(FONT_COLOR, 255),
                ..Default::default()
            },
        );
    }

    fn draw_tetromino(&self, tetromino: Tetromino, phantom: bool, ui_grid_pos: Vec2) {
        for y in 0..4 {
            for x in 0..4 {
                if tetromino.shape()[y][x] {
                    let mut tetromino_pos = tetromino.pos + Vec2::new(GAME_SIDE_WIDTH, 0.0);
                    if ui_grid_pos != Vec2::ZERO {
                        tetromino_pos = Vec2::new(tetromino.ui_offset(), 0.0) + ui_grid_pos;
                    }
                    tetromino_pos += Vec2::new(x as f32, y as f32);
                    Self::draw_block(tetromino_pos, tetromino.color, phantom);
                }
            }
        }
    }

    fn draw_block(grid_pos: Vec2, color: (u8, u8, u8), phantom: bool) {
        let pos = get_pos(grid_pos);
        let color_var = get_color(color, 255);

        if phantom {
            draw_rectangle_lines(
                pos.x + 4.0,
                pos.y + 4.0,
                BLOCK_SIZE - 8.0,
                BLOCK_SIZE - 8.0,
                2.0,
                color_var,
            );
        } else {
            draw_rectangle(
                pos.x + 1.0,
                pos.y + 1.0,
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
                pos.x,
                pos.y,
                BLOCK_SIZE,
                BLOCK_SIZE,
                1.0,
                get_color(UI_COLOR, 255),
            );
            draw_rectangle(
                pos.x + 4.0,
                pos.y + 4.0,
                BLOCK_SIZE - 8.0,
                BLOCK_SIZE - 8.0,
                color_var,
            );
        }
    }
}
