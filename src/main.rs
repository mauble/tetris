mod assets;
mod game;
mod global;
mod renderer;
mod tetromino;

use crate::assets::Assets;
use crate::game::Game;
use crate::global::*;
use crate::renderer::Renderer;
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
    let assets = Assets::new().await;

    let renderer = Renderer::new(&assets, 0);

    let mut player = Game::new(CONTROLS_PLAYER1);

    loop {
        clear_background(get_color(UI_COLOR, 255));

        player.run();
        renderer.draw_game(&player);

        next_frame().await;
    }
}
