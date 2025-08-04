use std::{collections::VecDeque, fmt::Debug, fs};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::services::search::fuzzy_search;

/// max number of elements in the history
const MAX_LENGTH: usize = 10;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    id: String,
    pub value: String,
}

impl Item {
    fn new(value: String) -> Self {
        Item {
            id: generate_id(),
            value,
        }
    }
}

// TODO: update this function
fn generate_id() -> String {
    let mut rng = rand::rng();

    let id = rng.random_range(u8::MIN..=u8::MAX);

    format!("#{id}")
}

fn load_history() -> std::io::Result<Vec<Item>> {
    let file = fs::read("history.json").ok();

    match file {
        Some(buf) => {
            let raw_json = String::from_utf8_lossy(&buf);
            let parsed: Vec<Item> = serde_json::from_str(&raw_json)?;
            Ok(parsed)
        }
        None => Ok(Vec::new()),
    }
}

pub fn save_history(history: &[Item]) -> std::io::Result<()> {
    let parsed: String = serde_json::to_string(history).unwrap();
    fs::write("history.json", parsed)?; // TODO: handle persistent storage properly using `directories`

    Ok(())
}

pub fn save_item(value: &str) -> std::io::Result<()> {
    // TODO: ascertain the memory cost to convert to VecDeque and back to Vec
    let mut history = VecDeque::from(load_history()?);

    if history.len() >= MAX_LENGTH {
        // remove item when list is equal to max length
        history.pop_front();
    }

    let item = Item::new(value.into());

    history.push_back(item);

    save_history(&Vec::from(history))?;

    Ok(())
}

pub fn get_item(id: &str) -> std::io::Result<Option<Item>> {
    let history = load_history()?;

    Ok(history.into_iter().find(|item| item.id == *id))
}

pub fn list_items() -> std::io::Result<Vec<Item>> {
    let mut items = load_history()?;

    items.reverse();

    Ok(items)
}

pub fn clear_history() -> std::io::Result<()> {
    save_history(&Vec::new())
}

pub fn search(query: &str) -> std::io::Result<()> {
    let history = load_history()?;

    let filtered_result = history.iter().filter(|item| {
        let value = fuzzy_search(query, &item.value.split(" ").collect::<Vec<&str>>(), None);

        // TODO: sort by closest match first
        !value.is_empty() || item.id.contains(query)
    });

    println!("Searching for: {query}\n");

    for item in filtered_result {
        println!("id: {} value: {}", item.id, item.value);
    }

    Ok(())
}

pub fn get_last_item() -> std::io::Result<Option<Item>> {
    let mut history = load_history()?;

    Ok(history.pop())
}
