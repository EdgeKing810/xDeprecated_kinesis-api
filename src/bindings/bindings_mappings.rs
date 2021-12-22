use crate::mappings::{fetch_all_mappings, save_all_mappings, Mapping};
use wasm_bindgen::prelude::*;

pub fn get_mappings_from_str(mappings: &str) -> Vec<Mapping> {
    let mut all_mappings = Vec::<Mapping>::new();

    for mapping in mappings.split("\n").collect::<Vec<&str>>() {
        let tmp_mapping = Mapping::from_string(mapping);
        all_mappings.push(tmp_mapping);
    }

    all_mappings
}

pub fn turn_mappings_to_str(mappings: Vec<Mapping>) -> String {
    let mut stringified_mappings = String::new();

    for mapping in mappings {
        stringified_mappings = format!(
            "{}{}{}",
            stringified_mappings,
            if stringified_mappings.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Mapping::to_string(mapping.clone())
        );
    }

    stringified_mappings
}

#[wasm_bindgen]
pub fn create_mapping(mappings: &str, id: &str, file_name: &str) -> String {
    let mut all_mappings = get_mappings_from_str(mappings);
    let result = Mapping::create(&mut all_mappings, id, file_name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_mapping(mappings: &str, id: &str, file_name: &str) -> String {
    let mut all_mappings = get_mappings_from_str(mappings);
    let result = Mapping::update(&mut all_mappings, id, file_name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn remove_mapping(mappings: &str, id: &str) -> String {
    let mut all_mappings = get_mappings_from_str(mappings);
    let result = Mapping::remove(&mut all_mappings, id);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn fetch_mappings(path: &str, encryption_key: &str) -> String {
    let all_mappings = fetch_all_mappings(path, &encryption_key.to_string());

    let mut stringified_mappings = String::new();

    for mapping in all_mappings {
        stringified_mappings = format!(
            "{}{}{}",
            stringified_mappings,
            if stringified_mappings.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Mapping::to_string(mapping.clone())
        );
    }

    stringified_mappings
}

#[wasm_bindgen]
pub fn save_mappings(mappings: &str, path: &str, encryption_key: &str) {
    let all_mappings = get_mappings_from_str(mappings);

    save_all_mappings(&all_mappings, path, &encryption_key.to_string());
}
