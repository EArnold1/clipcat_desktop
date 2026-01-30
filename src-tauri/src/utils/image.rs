use std::{
    fs,
    path::{Path, PathBuf},
};

use image::{ImageFormat, RgbImage};
use image_hasher::{HasherConfig, ImageHash};

use crate::config::{APP_IDENTIFIER, IMAGES_FOLDER_PATH};

/// A function to create an image has from a path
pub fn create_image_hash(path: impl Into<PathBuf>) -> ImageHash {
    let hasher = HasherConfig::new().to_hasher();

    let img_one = image::open(clip_image_path(path.into())).expect("Image should exist in path");

    hasher.hash_image(&img_one)
}

pub fn save_image(rgb_image: RgbImage, path: impl Into<PathBuf>) {
    let image_path = clip_image_path(path.into());

    rgb_image
        .save_with_format(image_path, ImageFormat::Jpeg)
        .expect("should save file"); // TODO: properly handle error
}

fn app_image_dir() -> PathBuf {
    let mut dir: PathBuf = dirs::config_dir()
        .ok_or("Could not find config directory")
        .expect("should get home directory");

    dir.push(APP_IDENTIFIER);
    dir.push(IMAGES_FOLDER_PATH);

    fs::create_dir_all(&dir).expect("Should recursively create directories");

    dir
}

pub fn clip_image_path<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
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

pub fn remove_image<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut image_dir = app_image_dir();

    image_dir.push(path);

    if let Err(e) = fs::remove_file(&image_dir) {
        eprintln!("failed to remove file {:?}: {}", image_dir, e);
    }
}
