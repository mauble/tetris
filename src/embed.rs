use macroquad::prelude::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

pub struct Embed {
    pub font: Font,
}

impl Embed {
    pub async fn new() -> Self {
        let font = Self::initialize_font().await;
        Self { font }
    }

    fn get_asset(path: &str) -> Vec<u8> {
        Asset::get(path)
            .map(|f| f.data.into())
            .expect(&format!("File {} not found.", path))
    }

    async fn initialize_font() -> Font {
        let font_bytes = Self::get_asset("font.ttf");
        load_ttf_font_from_bytes(&font_bytes).expect("Failed to load font.")
    }
}
