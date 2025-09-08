use crate::board::Board;
use crate::embed::get_font;
use crate::global::*;
use crate::tetromino::Tetromino;
use macroquad::prelude::*;

pub fn draw_tetromino(tetromino: Tetromino, offset: u32) {
    for y in 0..4 {
        for x in 0..4 {
            if tetromino.shape[y][x] {
                draw_block(
                    tetromino.pos.x + x as f32 - tetromino.center_offset() + offset as f32,
                    tetromino.pos.y + y as f32,
                    tetromino.color,
                    tetromino.phantom,
                );
            }
        }
    }
}

pub fn draw_board(board: &Board) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            draw_block(
                x as f32 + board.offset as f32,
                y as f32,
                board.board[y][x],
                false,
            );
        }
    }

    draw_tetromino(board.piece, board.offset);
    draw_tetromino(board.phantom, board.offset);
    draw_tetromino(board.preview, board.offset);

    draw_ui_text("Score".to_string(), board.offset, 0.0);
    draw_ui_text(board.score.to_string(), board.offset, 1.5);
    draw_ui_text("Lines".to_string(), board.offset, 4.5);
    draw_ui_text(board.lines.to_string(), board.offset, 6.0);
    draw_ui_text("Level".to_string(), board.offset, 9.0);
    draw_ui_text(board.level.to_string(), board.offset, 10.5);
}

pub fn draw_ui_text(text: String, x_offset: u32, y: f32) {
    let font = get_font();
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

pub fn draw_block(x: f32, y: f32, color: (u8, u8, u8), phantom: bool) {
    let x_loc = x * BLOCK_SIZE;
    let y_loc = y * BLOCK_SIZE;

    let color_var = get_color(color, 255);

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
