#![allow(dead_code)]

use crate::io::{fetch_file, save_file};

#[derive(Default, Debug, Clone)]
pub struct Mapping {
    id: String,
    file_name: String,
}

impl Mapping {
    fn create_no_check(id: &str, file_name: &str) -> Mapping {
        Mapping {
            id: String::from(id.trim()),
            file_name: String::from(file_name.trim()),
        }
    }

    pub fn exist(all_mappings: &Vec<Mapping>, id: &str) -> bool {
        let mut found = false;
        for mapping in all_mappings.iter() {
            if mapping.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn create(
        all_mappings: &mut Vec<Mapping>,
        id: &str,
        file_name: &str,
    ) -> Result<(), String> {
        if (!String::from(id).chars().all(|c| c.is_ascii_alphanumeric()) && !id.contains("_"))
            || (!String::from(file_name)
                .chars()
                .all(|c| c.is_ascii_alphanumeric())
                && !file_name.contains(".txt"))
        {
            return Err(format!(
                "Invalid character (perhaps a symbol?) found in id or file_name ({})",
                id
            ));
        }

        let mut found = false;
        for mapping in all_mappings.iter() {
            if mapping.id == id || mapping.file_name == file_name {
                found = true;
                break;
            }
        }

        if found {
            return Err(format!("A similar Mapping already exists ({})", id));
        }

        let new_mapping = Mapping {
            id: String::from(id.trim()),
            file_name: String::from(file_name.trim()),
        };

        all_mappings.push(new_mapping);
        Ok(())
    }

    pub fn update(
        all_mappings: &mut Vec<Mapping>,
        id: &str,
        file_name: &str,
    ) -> Result<(), String> {
        if !String::from(id).chars().all(|c| c.is_ascii_alphanumeric())
            || (!String::from(file_name)
                .chars()
                .all(|c| c.is_ascii_alphanumeric())
                && !file_name.contains(".txt"))
        {
            return Err(format!(
                "Invalid character (perhaps a symbol?) found in id or file_name ({})",
                id
            ));
        }

        let mut found = false;
        for mapping in all_mappings.iter() {
            if mapping.id == id {
                found = true;
            }

            if mapping.file_name == file_name {
                return Err(format!(
                    "Another Mapping is already using this file_name ({}, {})",
                    id, file_name
                ));
            }
        }

        if !found {
            return Err(format!("No Mapping with this id found ({})", id));
        }

        let new_mappings = all_mappings
            .iter()
            .map(|mapping| {
                if mapping.id == id {
                    return Mapping {
                        id: mapping.id.trim().to_string(),
                        file_name: String::from(file_name.trim()),
                    };
                } else {
                    return mapping.clone();
                }
            })
            .collect::<Vec<Mapping>>();
        *all_mappings = new_mappings;

        Ok(())
    }

    pub fn remove(all_mappings: &mut Vec<Mapping>, id: &str) -> Result<(), String> {
        let mut found = false;
        for mapping in all_mappings.iter() {
            if mapping.id == id {
                found = true;
                break;
            }
        }

        if !found {
            return Err(format!("No Mapping with this id found ({})", id));
        }

        let new_mappings = all_mappings
            .iter()
            .filter(|mapping| mapping.id != id)
            .map(|mapping| mapping.clone())
            .collect::<Vec<Mapping>>();
        *all_mappings = new_mappings;

        Ok(())
    }

    pub fn get_file_name(self: &Self) -> String {
        self.file_name.clone()
    }

    pub fn to_string(mapping: Mapping) -> String {
        format!("{}={}", mapping.id, mapping.file_name)
    }

    pub fn from_string(mapping_str: &str) -> Mapping {
        let current_mapping = mapping_str.split("=").collect::<Vec<&str>>();

        Mapping::create_no_check(current_mapping[0], current_mapping[1])
    }
}

pub fn fetch_all_mappings(path: &str, encryption_key: &String) -> Vec<Mapping> {
    let all_mappings_raw = fetch_file(String::from(path), encryption_key);

    let individual_mappings = all_mappings_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_mappings: Vec<Mapping> = Vec::<Mapping>::new();

    for mapping in individual_mappings {
        let tmp_mapping = Mapping::from_string(mapping);
        final_mappings.push(tmp_mapping);
    }

    final_mappings
}

pub fn save_all_mappings(mappings: &Vec<Mapping>, path: &str, encryption_key: &String) {
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

    save_file(String::from(path), stringified_mappings, encryption_key);
    println!("Mappings saved!");
}

pub fn get_file_name(id: &str, mappings: &Vec<Mapping>) -> Result<String, String> {
    let mut path = String::new();
    let mut found = false;
    for mapping in mappings {
        if mapping.id == id {
            found = true;
            path = mapping.file_name.clone();
            break;
        }
    }

    if !found {
        return Err(String::from("Error: No mapping with this id"));
    }

    Ok(path)
}
