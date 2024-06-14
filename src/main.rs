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
    let font = load_ttf_font(FONT_PATH).await.unwrap();
    // let icon = load_image(ICON_PATH).await.unwrap();

    let mut board1 = Board::new(
        &font,
        vec![
            KeyCode::A,
            KeyCode::D,
            KeyCode::S,
            KeyCode::W,
            KeyCode::Space,
        ],
        0,
    );
    let mut board2 = Board::new(
        &font,
        vec![
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Down,
            KeyCode::Up,
            KeyCode::RightShift,
        ],
        1,
    );

    loop {
        clear_background(get_color(UI_COLOR, 255));

        board1.run();
        board2.run();

        next_frame().await;
    }
}
