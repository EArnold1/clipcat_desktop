use std::{fs, path::PathBuf};

use image::{ImageFormat, RgbImage};
use image_hasher::HasherConfig;

const IMAGE_DISTANCE_THRESHOLD: u32 = 10;

/// function to compare images by hash
/// using `10` as the threshold
pub fn compare_image(last_clipped_path: &str, new_clip_data: &RgbImage) -> bool {
    let hasher = HasherConfig::new().to_hasher();

    let image_path = app_image_dir(last_clipped_path);

    let img_one = image::open(image_path).expect("last image should exist");

    let hash1 = hasher.hash_image(&img_one);
    let hash2 = hasher.hash_image(new_clip_data);

    hash1.dist(&hash2) < IMAGE_DISTANCE_THRESHOLD
}

pub fn save_image(rgb_image: RgbImage, path: &str) {
    let image_path = app_image_dir(path);

    // create image dir if it doesn't exist
    rgb_image
        .save_with_format(image_path, ImageFormat::Jpeg)
        .expect("should save file"); // TODO: properly handle error
}

pub fn app_image_dir(path: &str) -> PathBuf {
    let mut dir: PathBuf = dirs::config_dir()
        .ok_or("Could not find config directory")
        .expect("should get home directory");

    // TODO: handle literal values well
    dir.push("com.arnold.clipcat_app");
    dir.push("images");
    dir.push(path);

    if let Some(parent) = dir.parent() {
        fs::create_dir_all(parent).unwrap();
    };

    dir
}
