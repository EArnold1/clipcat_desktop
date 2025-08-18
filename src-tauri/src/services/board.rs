use arboard::Clipboard;

use crate::store::Clip;

pub fn read_clipboard() -> Option<Clip> {
    let mut clipboard = Clipboard::new().ok()?;

    if let Ok(value) = clipboard.get_text() {
        let clip = Clip::new_text(value);
        Some(clip)
    } else if let Ok(_) = clipboard.get_image() {
        // process image
        None
    } else {
        None
    }
}

pub fn write_clipboard(clip: Clip) {
    // NOTE: this might error if clipboards are not supported
    let mut clipboard = Clipboard::new().expect("should create an instance of the clipboard");

    match clip {
        Clip::Image { .. } => {
            // process copy image
        }
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
