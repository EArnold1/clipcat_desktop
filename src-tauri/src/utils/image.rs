use image::{ImageFormat, RgbImage};
use image_hasher::HasherConfig;

const IMAGE_DISTANCE_THRESHOLD: u32 = 10;

pub fn compare_image(last_clipped_path: &str, new_clip_data: &RgbImage) -> bool {
    let hasher = HasherConfig::new().to_hasher();

    let img_one =
        image::open(format!("images/{}", last_clipped_path)).expect("last image should exist");

    let hash1 = hasher.hash_image(&img_one);
    let hash2 = hasher.hash_image(new_clip_data);

    hash1.dist(&hash2) < IMAGE_DISTANCE_THRESHOLD
}

pub fn save_image(rgb_image: RgbImage, path: &str) {
    // create image dir if it doesn't exist
    rgb_image
        .save_with_format(format!("images/{}", path), ImageFormat::Jpeg)
        .expect("should save file"); // TODO: properly handle error
}
