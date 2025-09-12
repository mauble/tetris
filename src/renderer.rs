use crate::assets::Assets;
use crate::game::Game;
use crate::global::*;
use crate::tetromino::Tetromino;
use macroquad::prelude::*;

pub struct Renderer<'a> {
    assets: &'a Assets,
    controls: &'a Controls,
    player_offset: u32,
    pub paused: bool,
}

impl<'a> Renderer<'a> {
    pub fn new(assets: &'a Assets, controls: &'a Controls, player_offset: u32) -> Self {
        Self {
            assets,
            controls,
            player_offset: player_offset * GAME_WIDTH as u32,
            paused: false,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(self.controls.pause) {
            self.paused = !self.paused;
        }
    }

    pub fn draw(&self, game: &Game) {
        self.draw_game(&game);
        self.draw_paused();
    }

    fn draw_game(&self, game: &Game) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                Self::draw_block(
                    Vec2::new(x as f32 + self.player_offset as f32, y as f32),
                    game.board[y][x],
                    false,
                );
            }
        }

        self.draw_tetromino(game.piece, false, false);
        self.draw_tetromino(game.phantom, true, false);
        self.draw_tetromino(game.preview, false, true);

        let x_offset = BOARD_WIDTH as f32 + 2.5 + self.player_offset as f32;
        let y_offset = TETROMINO_PREVIEW_POS.y + 5.5;

        self.draw_text("Score", Vec2::new(x_offset, y_offset), true);
        self.draw_text(
            &game.score.to_string(),
            Vec2::new(x_offset, y_offset + 1.5),
            true,
        );
        self.draw_text("Lines", Vec2::new(x_offset, y_offset + 4.0), true);
        self.draw_text(
            &game.lines.to_string(),
            Vec2::new(x_offset, y_offset + 5.5),
            true,
        );
        self.draw_text("Level", Vec2::new(x_offset, y_offset + 8.0), true);
        self.draw_text(
            &game.level.to_string(),
            Vec2::new(x_offset, y_offset + 9.5),
            true,
        );
    }

    fn draw_paused(&self) {
        if self.paused {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.9),
            );

            self.draw_text(
                "Pause",
                Vec2::new(screen_width(), screen_height()) * 0.5,
                false,
            );
        }
    }

    fn draw_text(&self, text: &str, pos: Vec2, grid: bool) {
        let mut _pos = pos;
        if grid {
            _pos *= BLOCK_SIZE;
        }
        _pos -= get_text_center(&text, Some(&self.assets.font), FONT_SIZE, 1.0, 0.0);
        draw_text_ex(
            &text,
            _pos.x,
            _pos.y,
            TextParams {
                font_size: FONT_SIZE,
                font: Some(&self.assets.font),
                color: get_color(FONT_COLOR, 255),
                ..Default::default()
            },
        );
    }

    fn draw_tetromino(&self, tetromino: Tetromino, phantom: bool, preview: bool) {
        for y in 0..4 {
            for x in 0..4 {
                if tetromino.shape()[y][x] {
                    let mut preview_offset = 0.0;
                    if preview {
                        preview_offset = tetromino.preview_offset();
                    }
                    Self::draw_block(
                        Vec2::new(
                            tetromino.pos.x + x as f32 + preview_offset + self.player_offset as f32,
                            tetromino.pos.y + y as f32,
                        ),
                        tetromino.color,
                        phantom,
                    );
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
