use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, time::Duration};

use tauri::{AppHandle, Emitter, Manager, RunEvent};

use crate::services::board::read_clipboard;
use crate::store::{Clip, ClipsStore};

fn watcher(sender: Option<Arc<Sender<Clip>>>, app_handle: AppHandle) {
    let tx_option = match sender {
        Some(transmitter) => Some(Arc::clone(&transmitter)),
        None => None,
    };

    thread::spawn(move || {
        let delay = 5; // TODO: add settings file so this can be dynamic

        loop {
            let (incoming_clip, image) = read_clipboard();

            if let Some(clip) = incoming_clip {
                if let Clip::Text { value, .. } = &clip {
                    if value.trim().is_empty() {
                        thread::sleep(Duration::from_secs(delay));
                        continue;
                    }
                }

                let store_lock = app_handle.state::<Mutex<ClipsStore>>();
                let mut clip_store = store_lock.lock().expect("should acquire lock on store");

                if !clip_store.is_clipped(&clip, image) {
                    let start = Instant::now();
                    let item = clip_store.save_clip(clip);
                    if let Some(tx) = tx_option.as_ref() {
                        if let Err(e) = tx.send(item) {
                            eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                            break; // this stops the entire process
                        }
                    }
                    let elapsed = start.elapsed();

                    println!("time :{:?}", elapsed);
                } else {
                    thread::sleep(Duration::from_secs(delay));
                }
            }

            thread::sleep(Duration::from_secs(delay));
        }
    });
}

pub fn background_watcher(app_handle: &AppHandle, event: RunEvent) {
    {
        // background task for when UI is closed
        if let tauri::RunEvent::ExitRequested { api, .. } = &event {
            api.prevent_exit();

            let app_clone = app_handle.clone();

            thread::spawn(move || {
                watcher(None, app_clone);
            })
            .join()
            .expect("should join on the associated thread");
        }
    }

    {
        // background task for when app is loaded
        if matches!(&event, tauri::RunEvent::Ready) {
            let (tx, rx) = mpsc::channel();
            let channel = Arc::new(tx);

            watcher(Some(Arc::clone(&channel)), app_handle.clone());

            // using a thread so it doesn't block the event loop and stop the ui from rendering
            let handle = app_handle.clone();

            thread::spawn(move || {
                for value in rx {
                    handle.emit("new_clip", value).expect("should emit event");
                }
            });
        }
    }
}
