use lazy_static::lazy_static;
use macroquad::prelude::*;
use rust_embed::RustEmbed;
use std::sync::Mutex;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

lazy_static! {
    static ref FONT: Mutex<Option<Font>> = Mutex::new(None);
}

pub fn get_font() -> Font {
    FONT.lock().unwrap().clone().unwrap()
}

pub async fn initialize_assets() {
    initialize_font().await;
}

async fn initialize_font() {
    let font_bytes = Asset::get("font.ttf").expect("File font.ttf not found in embedded assets.");
    let font = load_ttf_font_from_bytes(font_bytes.data.as_ref()).unwrap();
    *FONT.lock().unwrap() = Some(font);
}
