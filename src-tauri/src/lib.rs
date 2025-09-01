mod services;
mod store;
mod utils;

use services::{background::background_watcher, board::write_clipboard};
use std::sync::Mutex;
use store::{ClipsData, ClipsStore};
use tauri::{AppHandle, Manager};
use utils::error::emit_error;

use crate::{services::board::clear_board, store::Clip};

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
            Ok(Some(clip)) => {
                if let Clip::Image { path } = &clip {
                    lock.set_last_clipped_path(path.clone());
                }
                write_clipboard(clip);
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
fn pin_clip(app: AppHandle, id: String) {
    let store = &app.state::<Mutex<ClipsStore>>();

    if let Ok(mut lock) = store.lock() {
        lock.pin_clip(&id);
    };
}

#[tauri::command]
fn unpin_clip(app: AppHandle, id: String) {
    let store = &app.state::<Mutex<ClipsStore>>();

    if let Ok(mut lock) = store.lock() {
        lock.unpin_clip(&id);
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

#[tauri::command]
fn delete_clip(app: AppHandle, id: String) {
    let store = &app.state::<Mutex<ClipsStore>>();

    if let Ok(mut lock) = store.lock() {
        lock.delete_mem_clip(&id);
        clear_board();
    };
}

#[tauri::command]
fn search_clips(app: AppHandle, query: String) -> Option<Vec<Clip>> {
    let store = &app.state::<Mutex<ClipsStore>>();

    let result = match store.lock() {
        Ok(store_lock) => Some(store_lock.search(&query)),
        Err(_) => None,
    };

    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let mut store = ClipsStore::new();
            store.remove_images(); // calling here to avoid deleting incoming image on clipboard, if any
            app.manage(Mutex::new(store));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_clips,
            copy_clip,
            clear_clips,
            pin_clip,
            unpin_clip,
            delete_clip,
            search_clips
        ])
        .build(tauri::generate_context!())
        .expect("error while running clipcat application")
        .run(background_watcher);
}
