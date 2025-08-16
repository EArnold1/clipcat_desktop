mod services;
mod store;
mod utils;

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
    utils::error::emit_error,
};

#[tauri::command]
fn load_clips(app: AppHandle) -> ClipsData {
    let store = app.state::<Mutex<ClipsStore>>();

    // if let Ok(mut lock) = store.lock() {
    //     match lock.load_clips() {
    //         Ok(clips) => {
    //             return clips;
    //         }
    //         Err(e) => {
    //             // emit error
    //             emit_error(&app, "failed to get clip");
    //             eprintln!("an error occurred while getting clip {:?}", e);

    //             return ClipsData::default();
    //         }
    //     }
    // };

    let clips = store
        .lock()
        .expect("should get lock on store")
        .load_clips()
        .unwrap_or_default();

    clips
}

#[tauri::command]
fn copy_clip(app: AppHandle, id: String) {
    let store = &app.state::<Mutex<ClipsStore>>();

    if let Ok(mut lock) = store.lock() {
        match lock.get_clip(&id) {
            Ok(Some(Item { value, .. })) => {
                write_clipboard(&value);
            }
            Err(e) => {
                // emit error
                emit_error(&app, "failed to get clip");
                eprintln!("an error occurred while getting clip {:?}", e);
            }
            _ => {}
        }
    };
}

#[tauri::command]
fn clear_clips(app: AppHandle) {
    let store = app.state::<Mutex<ClipsStore>>();

    store
        .lock()
        .expect("should get lock on store")
        .clear_clips();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(ClipsStore::new()));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_clips, copy_clip, clear_clips])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            // background task for when app is closed
            if let tauri::RunEvent::ExitRequested { api, .. } = &event {
                api.prevent_exit();

                let app_clone = app.clone();

                thread::spawn(move || {
                    watcher(None, app_clone);
                })
                .join()
                .expect("should join on the associated thread");
            }

            // background task for when app is loaded
            if matches!(&event, tauri::RunEvent::Ready) {
                let (tx, rx) = mpsc::channel();
                let channel = Arc::new(tx);

                watcher(Some(Arc::clone(&channel)), app.clone());

                // using a thread so it doesn't block the event loop and stop the ui from rendering
                let app_handle = app.clone();

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
