use std::{fs, path::PathBuf};

use image::{ImageFormat, RgbImage};
use image_hasher::HasherConfig;

const IMAGE_DISTANCE_THRESHOLD: u32 = 10;

/// function to compare images by hash
/// using `10` as the distance threshold
pub fn image_match(last_clipped_path: &str, new_clip_data: &RgbImage) -> bool {
    let hasher = HasherConfig::new().to_hasher();

    let img_one = image::open(clip_image_path(last_clipped_path)).expect("last image should exist");

    let hash1 = hasher.hash_image(&img_one);
    let hash2 = hasher.hash_image(new_clip_data);

    hash1.dist(&hash2) < IMAGE_DISTANCE_THRESHOLD
}

pub fn save_image(rgb_image: RgbImage, path: &str) {
    let image_path = clip_image_path(path);

    // create image dir if it doesn't exist
    rgb_image
        .save_with_format(image_path, ImageFormat::Jpeg)
        .expect("should save file"); // TODO: properly handle error
}

fn app_image_dir() -> PathBuf {
    let mut dir: PathBuf = dirs::config_dir()
        .ok_or("Could not find config directory")
        .expect("should get home directory");

    // TODO: handle literal values well
    dir.push("com.arnold.clipcat_app");
    dir.push("images");

    if let Some(parent) = dir.parent() {
        fs::create_dir_all(parent).unwrap();
    };

    dir
}

pub fn clip_image_path(path: &str) -> PathBuf {
    let mut app_image_dir = app_image_dir();

    app_image_dir.push(path);

    app_image_dir
}

pub fn clear_images(paths: &[PathBuf]) {
    let images_dir = app_image_dir();

    let entries = match fs::read_dir(&images_dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if paths.is_empty() || !paths.contains(&path) {
            if let Err(e) = fs::remove_file(&path) {
                eprintln!("failed to remove file {:?}: {}", path, e);
            }
        }
    }
}

pub fn remove_image(path: PathBuf) {
    let mut image_dir = app_image_dir();

    image_dir.push(path);

    if let Err(e) = fs::remove_file(&image_dir) {
        eprintln!("failed to remove file {:?}: {}", image_dir, e);
    }
}
