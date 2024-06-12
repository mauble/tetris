mod board;
mod global;
mod tetromino;

use board::*;
use global::*;
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_resizable: false,
        window_width: GAME_WIDTH as i32 * BLOCK_SIZE as i32 * 2,
        window_height: BOARD_HEIGHT as i32 * BLOCK_SIZE as i32,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut board1 = Board::new(
        new_board(),
        new_tetromino(),
        vec![
            KeyCode::A,
            KeyCode::D,
            KeyCode::S,
            KeyCode::W,
            KeyCode::Space,
        ],
        0,
        None,
    );
    let mut board2 = Board::new(
        new_board(),
        new_tetromino(),
        vec![
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Down,
            KeyCode::Up,
            KeyCode::RightShift,
        ],
        1,
        None,
    );

    loop {
        clear_background(WHITE);

        board1.run();
        board2.run();

        next_frame().await;
    }
}
