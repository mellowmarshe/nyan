use std::{collections::HashMap, env::var, path::PathBuf};

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

    pub static ref BROWN_CATS: HashMap<String, RgbaImage> = {
        let mut cats = HashMap::new();

        cats.insert("brown".to_string(), load_from_memory(include_bytes!("../assets/brown/brown.png")).expect("Failed to load cat.").to_rgba8());
        cats.insert("brown_heterochromia".to_string(), load_from_memory(include_bytes!("../assets/brown/brown_heterochromia.png")).expect("Failed to load cat.").to_rgba8());

        cats.insert("transparent".to_string(), load_from_memory(include_bytes!("../assets/brown/transparent.png")).expect("Failed to load cat.").to_rgba8());
        cats.insert("transparent_heterochromia".to_string(), load_from_memory(include_bytes!("../assets/brown/transparent_heterochromia.png")).expect("Failed to load cat.").to_rgba8());

        cats
    };

    pub static ref GREY_CATS: HashMap<String, RgbaImage> = {
        let mut cats = HashMap::new();

        cats.insert("grey".to_string(), load_from_memory(include_bytes!("../assets/grey/grey.png")).expect("Failed to load cat.").to_rgba8());
        cats.insert("grey_heterochromia".to_string(), load_from_memory(include_bytes!("../assets/grey/grey_heterochromia.png")).expect("Failed to load cat.").to_rgba8());

        cats.insert("transparent".to_string(), load_from_memory(include_bytes!("../assets/grey/transparent.png")).expect("Failed to load cat.").to_rgba8());
        cats.insert("transparent_heterochromia".to_string(), load_from_memory(include_bytes!("../assets/grey/transparent_heterochromia.png")).expect("Failed to load cat.").to_rgba8());

        cats
    };

    pub static ref WHITE_CATS: HashMap<String, RgbaImage> = {
        let mut cats = HashMap::new();

        cats.insert("white".to_string(), load_from_memory(include_bytes!("../assets/white/white.png")).expect("Failed to load cat.").to_rgba8());
        cats.insert("white_heterochromia".to_string(), load_from_memory(include_bytes!("../assets/white/white_heterochromia.png")).expect("Failed to load cat.").to_rgba8());

        cats.insert("transparent".to_string(), load_from_memory(include_bytes!("../assets/white/transparent.png")).expect("Failed to load cat.").to_rgba8());
        cats.insert("transparent_heterochromia".to_string(), load_from_memory(include_bytes!("../assets/white/transparent_heterochromia.png")).expect("Failed to load cat.").to_rgba8());

        cats
    };




    // // Normal cat images
    // pub static ref BROWN_CAT_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/brown/brown.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref WHITE_CAT_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/white/white.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref GREY_CAT_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/grey/grey.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // // Normal heterochromic images
    // pub static ref BROWN_HETEROCHROMIA_CAT_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/brown/brown_heterochromia.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref WHITE_HETEROCHROMIA_CAT_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/white/white_heterochromia.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref GREY_HETEROCHROMIA_CAT_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/grey/grey_heterochromia.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // // Emote normal images
    // pub static ref BROWN_EMOTE_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/brown/brown_emote.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref WHITE_EMOTE_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/white/white_emote.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref GREY_EMOTE_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/grey/grey_emote.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // // Emote heterochromic images
    // pub static ref BROWN_EMOTE_HETEROCHROMIC_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/brown/brown_heterochromia_emote.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref WHITE_EMOTE_HETEROCHROMIC_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/white/white_heterochromia_emote.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
    // pub static ref GREY_EMOTE_HETEROCHROMIC_IMAGE: RgbaImage =
    //     load_from_memory(include_bytes!("../assets/grey/grey_heterochromia_emote.png"))
    //         .expect("couldnt load house")
    //         .to_rgba8();
}
