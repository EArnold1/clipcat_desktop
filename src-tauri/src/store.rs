use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs, io};

use crate::{
    services::board::clear_board,
    store::generator::{generate_id, generate_path},
};

mod generator {
    use rand::{distr::Alphanumeric, Rng};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_string(length: Option<usize>) -> String {
        // time in milliseconds
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis();

        // random alphanumeric part (8 chars)
        let rand_part: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(length.unwrap_or(8))
            .map(char::from)
            .collect();

        format!("{}-{}", timestamp, rand_part)
    }

    pub fn generate_id() -> String {
        unique_string(Some(5))
    }

    pub fn generate_path() -> String {
        format!("{}.jpeg", unique_string(Some(7)))
    }
}

/// max number of elements in the history
const MAX_LENGTH: usize = 10;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Clip {
    Image { path: String },
    Text { id: String, value: String },
}

impl Clip {
    pub fn new_image() -> Self {
        Clip::Image {
            path: generate_path(),
        }
    }

    pub fn new_text(value: String) -> Self {
        Clip::Text {
            id: generate_id(),
            value,
        }
    }

    fn compare_clip(&self, other: &Self) -> bool {
        match (self, other) {
            (Clip::Text { value, .. }, Clip::Text { value: content, .. }) => value == content,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClipsData {
    pinned_clips: Vec<Clip>,
    mem_clips: Vec<Clip>,
}

#[derive(Clone, Debug)]
pub struct ClipsStore {
    clips: Vec<Clip>,
}

impl ClipsStore {
    pub fn new() -> Self {
        ClipsStore { clips: Vec::new() }
    }

    pub fn save_clip(&mut self, clip: Clip) -> Clip {
        let clips = &mut self.clips;

        if clips.len() >= MAX_LENGTH {
            // remove item when list is equal to max length
            clips.remove(0);
        };

        clips.push(clip.clone());

        clip
    }

    pub fn get_clip(&mut self, clip_id: &str) -> io::Result<Option<Clip>> {
        let clips = self.load_clips()?;

        let list = [clips.pinned_clips, clips.mem_clips].concat();

        Ok(list.into_iter().find(|clip| match clip {
            Clip::Image { path } => clip_id == path, // for images the path is used as id
            Clip::Text { id, .. } => clip_id == id,
        }))
    }

    /// checks if clip is already in store
    pub fn is_clipped(&mut self, new_clip: &Clip) -> bool {
        let clips = self.load_clips().expect("should return clips");

        let existing = [clips.pinned_clips, clips.mem_clips]
            .concat()
            .into_iter()
            .find(|clip| clip.compare_clip(new_clip));

        existing.is_some()
    }

    pub fn load_clips(&mut self) -> io::Result<ClipsData> {
        let pinned_clips = ClipsStore::get_pinned_clips()?;

        let mut mem_clips = self.clips.clone();

        mem_clips.reverse();

        Ok(ClipsData {
            mem_clips,
            pinned_clips,
        })
    }

    pub fn clear_clips(&mut self) {
        clear_board();
        self.clips.clear();
    }

    pub fn get_pinned_clips() -> io::Result<Vec<Clip>> {
        let file = fs::read("history.json").ok();

        match file {
            Some(buf) => {
                let raw_json = String::from_utf8_lossy(&buf);
                let mut parsed: Vec<Clip> = serde_json::from_str(&raw_json)?;
                parsed.reverse();
                Ok(parsed)
            }
            None => Ok(Vec::new()),
        }
    }

    // pub fn search(&self, query: &str) -> std::io::Result<()> {
    //     let clips = &self.clips;

    //     let filtered_result = clips.iter().filter(|item| {
    //         let value = fuzzy_search(query, &item.value.split(' ').collect::<Vec<&str>>(), None);

    //         // TODO: sort by closest match first
    //         !value.is_empty() || item.id.contains(query)
    //     });

    //     println!("Searching for: {query}\n");

    //     for item in filtered_result {
    //         println!("id: {} value: {}", item.id, item.value);
    //     }

    //     Ok(())
    // }
}
