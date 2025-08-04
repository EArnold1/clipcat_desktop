use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::{thread, time::Duration};

use crate::services::board::read_clipboard;
use crate::store::{get_last_item, save_item};

pub fn watcher(sender: Option<Arc<Sender<String>>>) {
    let tx_option = match sender {
        Some(transmitter) => Some(Arc::clone(&transmitter)),
        None => None,
    };

    thread::spawn(move || {
        loop {
            // Sleep for 5 seconds before polling the clipboard
            thread::sleep(Duration::from_secs(5));

            if let Some(value) = read_clipboard() {
                if let Ok(Some(item)) = get_last_item() {
                    if item.value != value {
                        match save_item(&value) {
                            Ok(_) => {
                                // using as_ref to get reference of the value in `tx_option`
                                // TODO: explain in details the use of as_ref
                                if let Some(tx) = tx_option.as_ref() {
                                    if let Err(e) = tx.send(value) {
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
                    } else {
                        continue;
                    }
                }
            }
        }
    });
}
