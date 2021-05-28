use image::{imageops, png::PngEncoder, ImageBuffer, ImageError, Pixel, Rgba, RgbaImage};
use std::ops::Deref;

use crate::{
    constants,
    models::cat::{Area, Cat, Color},
};

pub fn encode<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut bytes = Vec::new();
    let encoder = PngEncoder::new(&mut bytes);
    encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(bytes)
}

pub fn get_cat_picture(cat: &Cat) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    match cat.color {
        Color::Brown => {
            if cat.heterochromia {
                constants::BROWN_HETEROCHROMIA_CAT_IMAGE.clone()
            } else {
                constants::BROWN_CAT_IMAGE.clone()
            }
        }
        Color::Grey => {
            if cat.heterochromia {
                constants::GREY_HETEROCHROMIA_CAT_IMAGE.clone()
            } else {
                constants::GREY_CAT_IMAGE.clone()
            }
        }
        Color::White => {
            if cat.heterochromia {
                constants::WHITE_HETEROCHROMIA_CAT_IMAGE.clone()
            } else {
                constants::WHITE_CAT_IMAGE.clone()
            }
        }
    }
}

pub fn get_transparent_cat_picture(cat: &Cat) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    match cat.color {
        Color::Brown => {
            if cat.heterochromia {
                constants::BROWN_EMOTE_HETEROCHROMIC_IMAGE.clone()
            } else {
                constants::BROWN_EMOTE_IMAGE.clone()
            }
        }
        Color::Grey => {
            if cat.heterochromia {
                constants::GREY_EMOTE_HETEROCHROMIC_IMAGE.clone()
            } else {
                constants::GREY_EMOTE_IMAGE.clone()
            }
        }
        Color::White => {
            if cat.heterochromia {
                constants::WHITE_EMOTE_HETEROCHROMIC_IMAGE.clone()
            } else {
                constants::WHITE_EMOTE_IMAGE.clone()
            }
        }
    }
}

pub fn overlay_on_house(cat: &Cat) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let c: RgbaImage = get_transparent_cat_picture(cat);
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
