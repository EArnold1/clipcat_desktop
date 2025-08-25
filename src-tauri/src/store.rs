use image::RgbImage;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs, io, path::PathBuf, thread};

use crate::{
    services::board::clear_board,
    store::generator::{generate_id, generate_path},
    utils::image::{clear_images, compare_image, save_image},
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
    // write checks to make sure last clipped image path is properly updated
    last_clipped_image: Option<String>,
}

impl ClipsStore {
    pub fn new() -> Self {
        ClipsStore {
            clips: Vec::new(),
            last_clipped_image: None,
        }
    }

    pub fn save_clip(&mut self, clip: Clip) -> Clip {
        let clips = &mut self.clips;

        if clips.len() >= MAX_LENGTH {
            // remove item when list is equal to max length
            clips.remove(0);
        };

        if let Clip::Image { path } = &clip {
            self.last_clipped_image = Some(path.clone());
        };

        clips.push(clip.clone());

        clip
    }

    pub fn get_clip(&mut self, clip_id: &str) -> io::Result<Option<Clip>> {
        let clips = self.load_clips()?;

        let list = [clips.pinned_clips, clips.mem_clips].concat();

        Ok(list.into_iter().find(|clip| match clip {
            Clip::Image { path } => clip_id == path, //NOTE: for images the path is used as id
            Clip::Text { id, .. } => clip_id == id,
        }))
    }

    fn check_and_save_image(&mut self, path: String, image: Option<RgbImage>) -> bool {
        let Some(img) = image else {
            return false;
        };

        if let Some(last_path) = &self.last_clipped_image {
            if compare_image(last_path, &img) {
                return true;
            }
        }

        thread::spawn(move || {
            save_image(img, &path);
        });

        false
    }

    fn check_text_clip(&mut self, new_clip: &Clip) -> bool {
        let clips = self.load_clips().expect("failed to load clips");

        clips
            .pinned_clips
            .into_iter()
            .chain(clips.mem_clips)
            .any(|clip| clip.compare_clip(new_clip))
    }

    /// Checks if a clip is already in the store.
    /// For images, compares against last clipped image, and saves if it's new.
    /// While for texts, it checks if the value already exists in pinned or memory clips.
    pub fn is_clipped(&mut self, new_clip: &Clip, image: Option<RgbImage>) -> bool {
        if let Clip::Image { path } = new_clip {
            return self.check_and_save_image(path.clone(), image);
        }

        self.check_text_clip(new_clip)
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

    /// remove saved images that are no longer in clips store
    pub fn remove_images(&self) {
        let image_paths = self
            .clips // mem clips & pinned clips
            .iter()
            .filter_map(|clip| {
                if let Clip::Image { path } = clip {
                    Some(PathBuf::from(path))
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>();

        clear_images(&image_paths); // spawn thread?
    }

    pub fn clear_clips(&mut self) {
        clear_board();
        self.clips.clear();
        self.last_clipped_image = None;
        self.remove_images();
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

    pub fn set_last_clipped_path(&mut self, path: String) {
        self.last_clipped_image = Some(path);
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
