mod services;
mod store;

use crate::store::Item;

#[tauri::command]
fn load_clips() -> Vec<Item> {
    let clips = store::list_items();

    clips.unwrap_or_default()

    // Emit error if loading clips returns one
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_clips])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
