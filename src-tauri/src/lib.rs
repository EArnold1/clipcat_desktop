mod services;
mod store;

use store::{get_item, list_items, Item};

use crate::services::board::write_clipboard;

#[tauri::command]
fn load_clips() -> Vec<Item> {
    let clips = list_items();

    clips.unwrap_or_default()

    // Emit error if loading clips returns one
}

#[tauri::command]
fn copy_clip(id: String) {
    // if let Some(Some(Item { value, .. })) = get_item(&id).ok() {
    //     write_clipboard(&value);
    // }

    // if let Ok(item) = get_item(&id) {
    //     if let Some(Item { value, .. }) = item {
    //         write_clipboard(&value);
    //     }
    // };

    if let Ok(Some(Item { value, .. })) = get_item(&id) {
        write_clipboard(&value)
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_clips, copy_clip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
