use std::{env::var, path::PathBuf};

use crate::config::Config;
use image::{load_from_memory, RgbaImage};
use lazy_static::lazy_static;


lazy_static! {
    pub static ref CONFIG: Config = Config::new(PathBuf::from(if var("PRODUCTION").is_ok() {
        ".production.toml"
    } else {
        ".config.toml"
    }));
    pub static ref HOUSE_IMAGE: RgbaImage = load_from_memory(include_bytes!("../assets/house.png"))
        .expect("couldnt load house")
        .to_rgba8();
    // Normal cat images
    pub static ref BROWN_CAT_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/brown/brown.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref WHITE_CAT_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/white/white.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref GREY_CAT_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/grey/grey.png"))
            .expect("couldnt load house")
            .to_rgba8();
    // Normal heterochromic images
    pub static ref BROWN_HETEROCHROMIA_CAT_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/brown/brown_heterochromia.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref WHITE_HETEROCHROMIA_CAT_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/white/white_heterochromia.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref GREY_HETEROCHROMIA_CAT_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/grey/grey_heterochromia.png"))
            .expect("couldnt load house")
            .to_rgba8();
    // Emote normal images
    pub static ref BROWN_EMOTE_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/brown/brown_emote.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref WHITE_EMOTE_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/white/white_emote.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref GREY_EMOTE_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/grey/grey_emote.png"))
            .expect("couldnt load house")
            .to_rgba8();
    // Emote heterochromic images
    pub static ref BROWN_EMOTE_HETEROCHROMIC_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/brown/brown_heterochromia_emote.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref WHITE_EMOTE_HETEROCHROMIC_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/white/white_heterochromia_emote.png"))
            .expect("couldnt load house")
            .to_rgba8();
    pub static ref GREY_EMOTE_HETEROCHROMIC_IMAGE: RgbaImage =
        load_from_memory(include_bytes!("../assets/grey/grey_heterochromia_emote.png"))
            .expect("couldnt load house")
            .to_rgba8();
}
