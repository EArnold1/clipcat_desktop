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
    let store = app.state::<Mutex<ClipsStore>>();

    let clips = store
        .lock()
        .expect("should get lock on store")
        .load_clips()
        .unwrap_or_default();

    clips
}

#[tauri::command]
fn copy_clip(app: AppHandle, id: String) {
    let store = app.state::<Mutex<ClipsStore>>();

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

            // {
            //     // if matches!(&event, tauri::RunEvent::Ready) {
            //     let (tx, rx) = mpsc::channel();
            //     let channel = Arc::new(tx);

            //     watcher(Some(Arc::clone(&channel)), app.handle().clone());

            //     // using a thread so it doesn't block the event loop and stop the ui from rendering
            //     let app_handle = app.handle().clone();

            //     thread::spawn(move || {
            //         for value in rx {
            //             app_handle
            //                 .emit("new_clip", value)
            //                 .expect("should emit event");
            //         }
            //     });
            //     // }
            // }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_clips, copy_clip, clear_clips])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            // background task for when app is closed
            // if let tauri::RunEvent::ExitRequested { api, .. } = &event {
            //     api.prevent_exit();

            //     let app_clone = app.clone();

            //     thread::spawn(move || {
            //         watcher(None, app_clone);
            //     })
            //     .join()
            //     .expect("should join on the associated thread");
            // }

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
