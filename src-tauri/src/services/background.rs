use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::{thread, time::Duration};

use crate::services::board::read_clipboard;
use crate::store::{get_last_item, save_item, Item};

pub fn watcher(sender: Option<Arc<Sender<Item>>>) {
    let tx_option = match sender {
        Some(transmitter) => Some(Arc::clone(&transmitter)),
        None => None,
    };

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1)); // TODO: add settings file so this can be dynamic

            if let Some(value) = read_clipboard() {
                if value.trim().is_empty() {
                    continue;
                }

                match get_last_item() {
                    Ok(Some(clip)) if clip.value == value => continue,
                    Ok(_) => {
                        match save_item(&value) {
                            Ok(item) => {
                                // using as_ref to get reference of the value in `tx_option`
                                if let Some(tx) = tx_option.as_ref() {
                                    if let Err(e) = tx.send(item) {
                                        eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                                        break; // this stops the entire process
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                                break; // this stops the entire process
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("an error occurred: {:?}", e);
                        break;
                    }
                }
            }
        }
    });
}
