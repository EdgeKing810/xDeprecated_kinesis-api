use crate::io::{fetch_file, save_file};

#[derive(Debug, Clone)]
pub struct Mapping {
    id: String,
    file_name: String,
}

impl Mapping {
    fn create_no_check(id: &str, file_name: &str) -> Mapping {
        Mapping {
            id: String::from(id),
            file_name: String::from(file_name),
        }
    }

    pub fn create(
        all_mappings: &mut Vec<Mapping>,
        id: &str,
        file_name: &str,
    ) -> Result<(), String> {
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
            id: String::from(id),
            file_name: String::from(file_name),
        };

        all_mappings.push(new_mapping);
        Ok(())
    }
}

pub fn fetch_all_mappings() -> Vec<Mapping> {
    let all_mappings_raw = fetch_file(String::from("mappings.txt"));
    let individual_mappings = all_mappings_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_mappings: Vec<Mapping> = Vec::<Mapping>::new();

    for mapping in individual_mappings {
        let current_mapping = mapping.split("=").collect::<Vec<&str>>();
        let tmp_mapping = Mapping::create_no_check(current_mapping[0], current_mapping[1]);
        final_mappings.push(tmp_mapping);
    }

    final_mappings
}

pub fn save_all_mappings(mappings: Vec<Mapping>) {
    let mut stringified_mappings = String::new();
    for mapping in mappings {
        stringified_mappings = format!(
            "{}{}{}={}",
            stringified_mappings,
            if stringified_mappings.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            mapping.id,
            mapping.file_name
        );
    }

    save_file(String::from("mappings.txt"), stringified_mappings);
    println!("Mappings saved!");
}
