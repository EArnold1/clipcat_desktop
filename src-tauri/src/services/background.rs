use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use tauri::{AppHandle, Manager};

use crate::services::board::read_clipboard;
use crate::store::{ClipsStore, Item};

pub fn watcher(sender: Option<Arc<Sender<Item>>>, app_handle: AppHandle) {
    let tx_option = match sender {
        Some(transmitter) => Some(Arc::clone(&transmitter)),
        None => None,
    };

    thread::spawn(move || {
        loop {
            if let Some(value) = read_clipboard() {
                if value.trim().is_empty() {
                    thread::sleep(Duration::from_secs(5)); // TODO: add settings file so this can be dynamic
                    continue;
                }

                let store_lock = app_handle.state::<Mutex<ClipsStore>>();
                let mut clip_store = store_lock.lock().expect("should acquire lock on store");

                if !clip_store.is_clipped(&value) {
                    let item = clip_store.save_clip(&value);
                    if let Some(tx) = tx_option.as_ref() {
                        if let Err(e) = tx.send(item) {
                            eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                            break; // this stops the entire process
                        }
                    }
                }
            }
        }
    });
}
