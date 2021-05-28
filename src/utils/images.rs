use image::{imageops, png::PngEncoder, ImageBuffer, ImageError, Rgba, RgbaImage};

use crate::{
    constants,
    models::cat::{Area, Cat, Color},
};

pub fn encode(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<Vec<u8>, ImageError> {
    let mut bytes = Vec::new();
    let encoder = PngEncoder::new(&mut bytes);

    encoder.encode(&img, img.width(), img.height(), image::ColorType::Rgba8)?;

    Ok(bytes)
}

pub fn get_cat_picture(cat: &Cat, transparent: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let collection = match cat.color {
        Color::Brown => constants::BROWN_CATS.clone(),
        Color::Grey => constants::GREY_CATS.clone(),
        Color::White => constants::WHITE_CATS.clone(),
    };

    let color = format!(
        "{}{}",
        if transparent {
            "transparent".to_string()
        } else {
            cat.color.to_string()
        },
        if cat.heterochromia {
            "_heterochromia"
        } else {
            ""
        }
    );

    collection.get(&color).unwrap().clone()
}

pub fn overlay_on_house(cat: &Cat) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let c: RgbaImage = get_cat_picture(cat, true);
    let resized = imageops::resize(&c, 128, 128, imageops::FilterType::Nearest);

    let replenish = constants::CONFIG.areas.get("replenish").unwrap();
    let battle = constants::CONFIG.areas.get("battle").unwrap();
    let rest = constants::CONFIG.areas.get("rest").unwrap();

    let (x, y) = match cat.area {
        Area::Replenish => (replenish.x, replenish.y),
        Area::Battle => (battle.x, battle.y),
        Area::Rest => (rest.x, rest.y),
    };

    let mut house = constants::HOUSE_IMAGE.clone();
    imageops::overlay(&mut house, &resized, x, y);

    house
}
