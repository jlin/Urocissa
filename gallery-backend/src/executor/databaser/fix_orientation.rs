use std::{collections::BTreeMap, error::Error, io, mem, path::Path};

use anyhow::Context;
use image::DynamicImage;

use crate::public::database_struct::database::definition::DataBase;

use super::{
    generate_dynamic_image::generate_dynamic_image,
    generate_exif::generate_exif,
    generate_width_height::{generate_img_width_height, generate_phash, generate_thumbhash},
};



pub fn fix_orientation(database: &mut DataBase, dynamic_image: &mut DynamicImage) -> () {
    if let Some(orientation) = database.exif_vec.get("Orientation") {
        match orientation.as_str() {
            "row 0 at right and column 0 at top" => {
                *dynamic_image = dynamic_image.rotate90();
                std::mem::swap(&mut database.width, &mut database.height)
            }
            "row 0 at bottom and column 0 at right" => *dynamic_image = dynamic_image.rotate180(),
            "row 0 at left and column 0 at bottom" => {
                *dynamic_image = dynamic_image.rotate270();
                std::mem::swap(&mut database.width, &mut database.height)
            }
            _ => (),
        }
    }
}
