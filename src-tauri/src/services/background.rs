use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use crate::services::board::read_clipboard;
use crate::store::{ClipsStore, Item};

pub fn watcher(sender: Option<Arc<Sender<Item>>>, store: Arc<Mutex<ClipsStore>>) {
    let tx_option = match sender {
        Some(transmitter) => Some(Arc::clone(&transmitter)),
        None => None,
    };

    let store_lock = Arc::clone(&store);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5)); // TODO: add settings file so this can be dynamic

            if let Some(value) = read_clipboard() {
                if value.trim().is_empty() {
                    continue;
                }

                let mut clip_store = store_lock.lock().expect("should get store lock");

                if !clip_store.is_clipped(&value) {
                    let item = clip_store.save_clip(&value);
                    if let Some(tx) = tx_option.as_ref() {
                        if let Err(e) = tx.send(item) {
                            eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                            break; // this stops the entire process
                        }
                    }
                }

                // match clip_store.is_clipped() {
                // Some(clip) if clip.value == value => continue,
                // Some(_) | None => {
                //     let item = clip_store.save_clip(&value);
                //     if let Some(tx) = tx_option.as_ref() {
                //         if let Err(e) = tx.send(item) {
                //             eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                //             break; // this stops the entire process
                //         }
                //     }

                // match clip_store.save_clip(&value) {
                //     Ok(item) => {
                //         // using as_ref to get reference of the value in `tx_option`
                //         if let Some(tx) = tx_option.as_ref() {
                //             if let Err(e) = tx.send(item) {
                //                 eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                //                 break; // this stops the entire process
                //             }
                //         }
                //     }
                //     Err(e) => {
                //         eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
                //         break; // this stops the entire process
                //     }
                // }
                // }
                // }
            }
        }
    });
}

// pub fn watcher(sender: Arc<Sender<String>>) {
//     let transmitter = Arc::clone(&sender);

//     thread::spawn(move || {
//         loop {
//             thread::sleep(Duration::from_secs(1)); // TODO: add settings file so this can be dynamic

//             if let Some(value) = read_clipboard() {
//                 if value.trim().is_empty() {
//                     continue;
//                 }

//                 if let Err(e) = transmitter.send(value) {
//                     eprintln!("an error occurred: {:?}", e); // FIXME: update error message and handle properly
//                     break; // this stops the entire process
//                 }
//             }
//         }
//     });
// }
