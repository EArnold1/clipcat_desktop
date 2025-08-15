mod services;
mod store;

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use store::Item;

use services::board::write_clipboard;
use tauri::{AppHandle, Emitter, Manager};

use crate::{
    services::background::watcher,
    store::{ClipsData, ClipsStore},
};

#[tauri::command]
fn load_clips(app: AppHandle) -> ClipsData {
    let store = app.state::<Arc<Mutex<ClipsStore>>>();

    let clips = store
        .lock()
        .expect("should get lock on store")
        .load_clips()
        .unwrap_or_default();

    clips
}

#[tauri::command]
fn copy_clip(app: AppHandle, id: String) {
    let store: tauri::State<'_, Arc<Mutex<ClipsStore>>> = app.state::<Arc<Mutex<ClipsStore>>>();

    if let Some(Item { value, .. }) = store
        .lock()
        .expect("should get lock on store")
        .get_clip(&id)
    {
        write_clipboard(&value)
    };
}

#[tauri::command]
fn clear_clips(app: AppHandle) {
    let store = app.state::<Arc<Mutex<ClipsStore>>>();

    store
        .lock()
        .expect("should get lock on store")
        .clear_clips();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Arc::new(Mutex::new(ClipsStore::new()))); // TODO: Remove `Arc` as it is not needed
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_clips, copy_clip, clear_clips])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            let store = app.state::<Arc<Mutex<ClipsStore>>>();

            // background task for when app is closed
            if let tauri::RunEvent::ExitRequested { api, .. } = &event {
                api.prevent_exit();
                let clip_store = Arc::clone(&store);

                thread::spawn(move || {
                    watcher(None, clip_store);
                })
                .join()
                .expect("should join on the associated thread");
            }

            // background task for when app is loaded
            if matches!(&event, tauri::RunEvent::Ready) {
                let (tx, rx) = mpsc::channel();

                let channel = Arc::new(tx);
                let app_handle = app.clone();
                let clip_store = Arc::clone(&store);

                watcher(Some(Arc::clone(&channel)), clip_store);

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
