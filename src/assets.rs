use macroquad::prelude::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

pub struct Assets {
    pub font: Font,
}

impl Assets {
    pub async fn new() -> Self {
        let font = Self::initialize_font().await;
        Self { font }
    }

    fn load_asset(path: &str) -> Vec<u8> {
        Asset::get(path)
            .map(|f| f.data.into())
            .expect(&format!("File {} not found.", path))
    }

    async fn initialize_font() -> Font {
        let font_bytes = Self::load_asset("font.ttf");
        load_ttf_font_from_bytes(&font_bytes).expect("Failed to load font.")
    }
}
