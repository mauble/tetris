use crate::embed::Embed;
use crate::game::Game;
use crate::global::*;
use crate::tetromino::Tetromino;
use macroquad::prelude::*;

pub struct Renderer {
    pub offset: u32,
    pub embed: Embed,
}

impl Renderer {
    pub async fn new(offset: u32) -> Self {
        let offset = offset * GAME_WIDTH as u32;
        let embed = Embed::new().await;
        Self { offset, embed }
    }

    pub fn draw_game(&self, game: &Game) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                Self::draw_block(
                    x as f32 + self.offset as f32,
                    y as f32,
                    game.board[y][x],
                    false,
                );
            }
        }

        self.draw_tetromino(game.piece);
        self.draw_tetromino(game.phantom);
        self.draw_tetromino(game.preview);

        let x_offset = BOARD_WIDTH as f32 + 2.5;
        let y_offset = TETROMINO_PREVIEW_POS.y + 6.0;

        self.draw_text("Score", 0.0, 0.0, x_offset, y_offset);
        self.draw_text(&game.score.to_string(), 0.0, 1.5, x_offset, y_offset);
        self.draw_text("Lines", 0.0, 4.0, x_offset, y_offset);
        self.draw_text(&game.lines.to_string(), 0.0, 5.5, x_offset, y_offset);
        self.draw_text("Level", 0.0, 8.0, x_offset, y_offset);
        self.draw_text(&game.level.to_string(), 0.0, 9.5, x_offset, y_offset);
    }

    fn draw_tetromino(&self, tetromino: Tetromino) {
        for y in 0..4 {
            for x in 0..4 {
                if tetromino.shape[y][x] {
                    Self::draw_block(
                        tetromino.pos.x + x as f32 - tetromino.center_offset() + self.offset as f32,
                        tetromino.pos.y + y as f32,
                        tetromino.color,
                        tetromino.phantom,
                    );
                }
            }
        }
    }

    fn draw_text(&self, text: &str, x: f32, y: f32, x_offset: f32, y_offset: f32) {
        let center = get_text_center(&text, Some(&self.embed.font), FONT_SIZE, 1.0, 0.0);
        draw_text_ex(
            &text,
            (x_offset + self.offset as f32 + x) * BLOCK_SIZE - center.x,
            (y_offset + y) * BLOCK_SIZE - center.y,
            TextParams {
                font_size: FONT_SIZE,
                font: Some(&self.embed.font),
                color: get_color(FONT_COLOR, 255),
                ..Default::default()
            },
        );
    }

    fn draw_block(x: f32, y: f32, color: (u8, u8, u8), phantom: bool) {
        let x_loc = x * BLOCK_SIZE;
        let y_loc = y * BLOCK_SIZE;

        let color_var = get_color(color, 255);

        if phantom {
            draw_rectangle_lines(
                x_loc + 4.0,
                y_loc + 4.0,
                BLOCK_SIZE - 8.0,
                BLOCK_SIZE - 8.0,
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
                get_color(UI_COLOR, 255),
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
}
