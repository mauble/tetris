mod embed;
mod game;
mod global;
mod renderer;
mod tetromino;

use game::Game;
use global::*;
use macroquad::prelude::*;
use renderer::Renderer;

fn conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_resizable: false,
        window_width: GAME_WIDTH as i32 * BLOCK_SIZE as i32,
        window_height: BOARD_HEIGHT as i32 * BLOCK_SIZE as i32,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let renderer = Renderer::new(0).await;

    let mut player = Game::new(vec![
        KeyCode::A,
        KeyCode::D,
        KeyCode::S,
        KeyCode::W,
        KeyCode::Space,
    ]);

    loop {
        clear_background(get_color(UI_COLOR, 255));

        player.run();
        renderer.draw_game(&player);

        next_frame().await;
    }
}
