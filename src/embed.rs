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

    pub async fn initialize_font() -> Font {
        let font_bytes = Asset::get("font.ttf")
            .expect("File font.ttf not found.")
            .data;
        load_ttf_font_from_bytes(&font_bytes).expect("Failed to load font.")
    }
}
