use arboard::{Clipboard, ImageData};
use image::{DynamicImage, ImageBuffer, RgbImage, RgbaImage};

use crate::{store::Clip, utils::image::clip_image_path};

pub fn read_clipboard() -> (Option<Clip>, Option<RgbImage>) {
    let mut clipboard = Clipboard::new().expect("should create an instance of the clipboard");

    if let Ok(image) = clipboard.get_image() {
        // process image
        let clip = Clip::new_image();

        (Some(clip), Some(read_image(image)))
    } else if let Ok(value) = clipboard.get_text() {
        let clip = Clip::new_text(value);

        (Some(clip), None)
    } else {
        (None, None)
    }
}

pub fn write_clipboard(clip: Clip) {
    // NOTE: this might error if clipboards are not supported
    let mut clipboard = Clipboard::new().expect("should create an instance of the clipboard");

    match clip {
        Clip::Image { path } => write_image(&mut clipboard, &path),
        Clip::Text { value, .. } => write_text(&mut clipboard, value),
    }
}

fn write_text(clipboard: &mut Clipboard, value: String) {
    clipboard
        .set_text(value)
        .expect("should set text to clipboard")
}

pub fn clear_board() {
    let mut clipboard = Clipboard::new().expect("should create an instance of the clipboard"); // NOTE: this might error if clipboards are not supported
    clipboard.clear().expect("should clear clipboard");
}

pub fn read_image(image: ImageData) -> RgbImage {
    let ImageData {
        height,
        width,
        bytes,
    } = image;

    let rgba: RgbaImage =
        ImageBuffer::from_raw(width as u32, height as u32, bytes.into_owned()).unwrap();

    DynamicImage::ImageRgba8(rgba).into_rgb8()
}

fn write_image(clipboard: &mut Clipboard, path: &str) {
    let dyn_img: DynamicImage = image::open(clip_image_path(path)).expect("should find image");

    // convert to RGBA8 pixel format
    let rgba: RgbaImage = dyn_img.to_rgba8();

    let image = ImageData {
        bytes: rgba.to_vec().into(),
        width: rgba.width() as usize,
        height: rgba.height() as usize,
    };

    clipboard.set_image(image).expect("should set image");
}
