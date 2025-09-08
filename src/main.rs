mod embed;
mod game;
mod global;
mod render;
mod tetromino;

use game::Game;
use global::*;
use macroquad::prelude::*;
use render::Renderer;

fn conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_resizable: false,
        window_width: GAME_WIDTH as i32 * BLOCK_SIZE as i32 * 1,
        window_height: BOARD_HEIGHT as i32 * BLOCK_SIZE as i32,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let renderer = Renderer::new().await;

    let mut player1 = Game::new(
        vec![
            KeyCode::A,
            KeyCode::D,
            KeyCode::S,
            KeyCode::W,
            KeyCode::Space,
        ],
        0,
    );

    // let mut player2 = Game::new(
    //     vec![
    //         KeyCode::Left,
    //         KeyCode::Right,
    //         KeyCode::Down,
    //         KeyCode::Up,
    //         KeyCode::RightShift,
    //     ],
    //     1,
    // );

    loop {
        clear_background(get_color(UI_COLOR, 255));

        player1.run();
        renderer.draw_game(&player1);

        // player2.run();

        next_frame().await;
    }
}
