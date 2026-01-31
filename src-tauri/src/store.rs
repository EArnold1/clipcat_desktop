use image::RgbImage;
use image_hasher::{HasherConfig, ImageHash};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    thread,
};

use crate::{
    config::{APP_IDENTIFIER, CLIPS_MAX_LENGTH, MAX_PIN_LENGTH, PINNED_CLIPS_FILE_NAME},
    services::{board::clear_board, search::fuzzy_search},
    store::generator::{generate_id, generate_path},
    utils::image::{clear_images, clip_image_path, create_image_hash, remove_image, save_image},
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Clip {
    Image {
        path: String, //TODO: should be file_name
    },
    Text {
        id: String,
        value: String,
    },
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

    fn compare_text_clip(&self, other: &Self) -> bool {
        match (self, other) {
            (Clip::Text { value, .. }, Clip::Text { value: content, .. }) => value == content,
            _ => false,
        }
    }

    fn id(&self) -> String {
        match self {
            Clip::Image { path } => path.clone(),
            Clip::Text { id, .. } => id.clone(),
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
    clips: HashMap<String, Clip>,
    image_set: HashSet<ImageHash>,
}

impl ClipsStore {
    pub fn new() -> Self {
        ClipsStore {
            clips: HashMap::with_capacity(CLIPS_MAX_LENGTH),
            image_set: HashSet::new(),
        }
    }

    fn pinned_path() -> PathBuf {
        let mut dir: PathBuf = dirs::config_dir()
            .ok_or("Could not find config directory")
            .expect("should get home directory");

        dir.push(APP_IDENTIFIER);

        dir.push(PINNED_CLIPS_FILE_NAME);

        dir
    }

    pub fn save_clip(&mut self, clip: Clip) -> Clip {
        let clips_capacity = &self.clips.capacity();

        if *clips_capacity >= CLIPS_MAX_LENGTH {
            if let Some((key, ..)) = self.clips.iter().next() {
                let key_to_delete = key.clone();
                self.delete_mem_clip(&key_to_delete);
            }
        };

        let key = match &clip {
            Clip::Image { path } => path.clone(),
            Clip::Text { id, .. } => id.clone(),
        };

        self.clips.insert(key, clip.clone());

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

    fn check_and_save_image(&mut self, path: &str, image: Option<RgbImage>) -> bool {
        let Some(img) = image else {
            return false;
        };

        let hasher = HasherConfig::new().to_hasher();
        let image_hash = hasher.hash_image(&img);

        if self.image_set.contains(&image_hash) {
            return true;
        }

        self.image_set.insert(image_hash);

        let image_path: PathBuf = [path].iter().collect();

        thread::spawn(move || {
            save_image(img, image_path);
        });

        false
    }

    fn check_text_clip(&mut self, new_clip: &Clip) -> bool {
        let clips = self.load_clips().expect("failed to load clips");

        clips
            .pinned_clips
            .into_iter()
            .chain(clips.mem_clips)
            .any(|clip| clip.compare_text_clip(new_clip))
    }

    /// Checks if a clip is already in the store.
    /// For images, compares against last clipped image, and saves if it's new.
    /// While for texts, it checks if the value already exists in pinned or memory clips.
    pub fn is_clipped(&mut self, new_clip: &Clip, image: Option<RgbImage>) -> bool {
        if let Clip::Image { path } = new_clip {
            return self.check_and_save_image(path, image);
        }

        self.check_text_clip(new_clip)
    }

    pub fn load_clips(&mut self) -> io::Result<ClipsData> {
        let pinned_clips = ClipsStore::get_pinned_clips()?;

        let mut mem_clips: Vec<Clip> = self.clips.clone().into_values().collect();

        mem_clips.reverse();

        Ok(ClipsData {
            mem_clips,
            pinned_clips,
        })
    }

    /// This function removes saved images that are no longer on the clip store
    pub fn remove_images(&mut self) {
        let clips = self.load_clips().expect("should return clips");

        let image_paths = clips
            .pinned_clips
            .into_iter()
            .chain(clips.mem_clips)
            .filter_map(|clip| {
                if let Clip::Image { path } = clip {
                    Some(clip_image_path(path))
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
        self.remove_images();
    }

    pub fn get_pinned_clips() -> io::Result<Vec<Clip>> {
        let path = ClipsStore::pinned_path();

        let file = fs::read(&path).ok();

        match file {
            Some(buf) => {
                let raw_json = String::from_utf8_lossy(&buf);
                let mut parsed: Vec<Clip> = serde_json::from_str(&raw_json)?;
                parsed.reverse();
                Ok(parsed)
            }
            None => {
                let mut file = File::create(&path)?;
                file.write_all(b"[]").unwrap();

                Ok(Vec::new())
            }
        }
    }

    pub fn pin_clip(&mut self, clip_id: &str) {
        let mut pinned_clips = ClipsStore::get_pinned_clips().expect("should get pinned clips");

        if pinned_clips.len() >= MAX_PIN_LENGTH {
            //TODO: should send alert saying `limit reached`
            return;
        }

        if let Some(clip) = self.clips.remove(clip_id) {
            pinned_clips.reverse();

            pinned_clips.push(clip);

            let serialized: String = serde_json::to_string(&pinned_clips).unwrap();
            fs::write(ClipsStore::pinned_path(), serialized).expect("should write to file");
        }
    }

    pub fn unpin_clip(&mut self, clip_id: &str) {
        let mut pinned_clips = ClipsStore::get_pinned_clips().expect("should get pinned clips");

        if let Some(index) = pinned_clips.iter().position(|clip| match clip {
            Clip::Image { path } => clip_id == path, //NOTE: for images the path is used as id
            Clip::Text { id, .. } => clip_id == id,
        }) {
            let clip = pinned_clips.remove(index);

            self.clips.insert(clip.id(), clip);

            let serialized: String = serde_json::to_string(&pinned_clips).unwrap();
            fs::write(ClipsStore::pinned_path(), serialized).expect("should write to file");
        }
    }

    pub fn delete_mem_clip(&mut self, clip_id: &str) {
        if let Some(Clip::Image { path }) = self.clips.remove(clip_id) {
            let image_hash = create_image_hash(&path);
            self.image_set.remove(&image_hash);

            remove_image(&path);
        }
    }

    /// Search mem text clips,
    /// this doesn't do image text search.
    pub fn search(&self, query: &str) -> Vec<Clip> {
        let clips: Vec<Clip> = self.clips.clone().into_values().collect();

        let mut filtered_result = clips
            .into_iter()
            .filter_map(|clip| {
                if let Clip::Text { value, id } = &clip {
                    let (search, dist) =
                        fuzzy_search(query, &value.split(' ').collect::<Vec<&str>>(), None);
                    if !search.is_empty() || id.contains(query) {
                        Some((clip, dist))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<(Clip, usize)>>();

        filtered_result.sort_by_key(|(_, dist)| *dist);

        filtered_result
            .into_iter()
            .map(|(clip, _)| clip)
            .collect::<Vec<Clip>>()
    }
}
