use arboard::Clipboard;

pub fn read_clipboard() -> Option<String> {
    let mut clipboard = Clipboard::new().ok()?;
    clipboard.get_text().ok()
}

pub fn write_clipboard(text: &str) {
    let mut clipboard = Clipboard::new().expect("should create an instance of the clipboard"); // NOTE: this might error if clipboards are not supported
    clipboard.set_text(text).unwrap();
}
