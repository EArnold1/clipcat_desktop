mod services;
mod store;

use std::{
    sync::{mpsc, Arc},
    thread,
};

use store::{get_item, list_items, Item};

use services::board::write_clipboard;
use tauri::Emitter;

use crate::services::background::watcher;

#[tauri::command]
fn load_clips() -> Vec<Item> {
    let clips = list_items();

    clips.unwrap_or_default()

    // Emit error if loading clips returns one
}

#[tauri::command]
fn copy_clip(id: String) {
    if let Ok(Some(Item { value, .. })) = get_item(&id) {
        write_clipboard(&value)
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_clips, copy_clip])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            // background task for when app is closed
            if let tauri::RunEvent::ExitRequested { api, .. } = &event {
                api.prevent_exit();
                thread::spawn(|| {
                    watcher(None);
                })
                .join()
                .expect("should join on the associated thread");
            }

            // background task for when app is loaded
            if let tauri::RunEvent::Ready = &event {
                let (tx, rx) = mpsc::channel();

                let channel = Arc::new(tx);
                let app_handle = app.clone();

                watcher(Some(Arc::clone(&channel)));

                // using a thread so it doesn't block the event loop and stop the ui from rendering
                thread::spawn(move || {
                    for value in rx {
                        app_handle
                            .emit("new_clip", value)
                            .expect("should emit event");
                    }
                });
            }
        });
}
