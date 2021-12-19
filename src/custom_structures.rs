use crate::structures::Structure;
// use crate::encryption::EncryptionKey;

#[derive(Default, Debug, Clone)]
pub struct CustomStructure {
    pub id: String,
    name: String,
    structures: Vec<Structure>,
}

impl CustomStructure {
    pub fn create(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &str,
        name: &str,
    ) -> Result<(), String> {
        // if Self::exist(all_custom_structures, id) {
        //     let new_id = EncryptionKey::generate_uuid(8);
        //     return Self::create(all_custom_structures, &*new_id.to_string(), name);
        // }

        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: String = String::new();

        let new_custom_structure = CustomStructure {
            id: tmp_id.clone(),
            name: "".to_string(),
            structures: vec![],
        };
        all_custom_structures.push(new_custom_structure);

        let id_update = Self::update_id(all_custom_structures, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("Error: {}", e);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let name_update = Self::update_name(all_custom_structures, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if has_error {
            let delete_project = Self::delete(all_custom_structures, &new_id);
            if let Err(e) = delete_project {
                println!("Error: {}", e);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_custom_structures: &Vec<CustomStructure>, id: &str) -> bool {
        let mut found = false;
        for custom_structure in all_custom_structures.iter() {
            if custom_structure.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_id(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        new_id: &str,
    ) -> Result<(), String> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == new_id {
                return Err(String::from("Error: id is already in use"));
            }
        }

        if !String::from(new_id)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(String::from("Error: new_id contains an invalid character"));
        }

        if String::from(new_id.trim()).len() < 1 {
            return Err(String::from(
                "Error: new_id does not contain enough characters",
            ));
        } else if String::from(new_id.trim()).len() > 100 {
            return Err(String::from("Error: new_id contains too many characters"));
        }

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                custom_structure.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_custom_structure {
            return Err(String::from("Error: Custom Structure not found"));
        }

        Ok(())
    }

    pub fn update_name(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        name: &str,
    ) -> Result<(), String> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        if !String::from(name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ' ')
        {
            return Err(String::from("Error: name contains an invalid character"));
        }

        if String::from(name.trim()).len() < 1 {
            return Err(String::from(
                "Error: name does not contain enough characters",
            ));
        } else if String::from(name.trim()).len() > 100 {
            return Err(String::from("Error: name contains too many characters"));
        }

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                custom_structure.name = name.trim().to_string();
                break;
            }
        }

        if let None = found_custom_structure {
            return Err(String::from("Error: Custom Structure not found"));
        }

        Ok(())
    }

    pub fn add_structure(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structure: Structure,
    ) -> Result<(), String> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());

                let mut current_structures = custom_structure.structures.clone();
                current_structures.push(structure);
                custom_structure.structures = current_structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err(String::from("Error: Custom Structure not found"));
        }

        Ok(())
    }

    pub fn update_structure(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structure: Structure,
    ) -> Result<(), String> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                let mut found_structure = false;

                let mut current_structures = custom_structure.structures.clone();

                for current_structure in current_structures.iter_mut() {
                    if current_structure.id == structure.id {
                        *current_structure = structure.clone();
                        found_structure = true;
                    }
                }

                if !found_structure {
                    current_structures.push(structure);
                }

                custom_structure.structures = current_structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err(String::from("Error: Custom Structure not found"));
        }

        Ok(())
    }

    pub fn set_structures(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structures: Vec<Structure>,
    ) -> Result<(), String> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                custom_structure.structures = structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err(String::from("Error: Custom Structure not found"));
        }

        Ok(())
    }

    pub fn remove_structure(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structure_id: &String,
    ) -> Result<(), String> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());

                let mut current_structures = custom_structure.structures.clone();
                let result_delete_structure =
                    Structure::delete(&mut current_structures, structure_id);
                if let Err(e) = result_delete_structure {
                    return Err(e);
                }
                custom_structure.structures = current_structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err(String::from("Error: Custom Structure not found"));
        }

        Ok(())
    }

    pub fn delete(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
    ) -> Result<(), String> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == id.to_string() {
                found_custom_structure = Some(custom_structure.clone());
                break;
            }
        }

        if let None = found_custom_structure {
            return Err(String::from("Error: Custom Structure not found"));
        }

        let updated_structures: Vec<CustomStructure> = all_custom_structures
            .iter_mut()
            .filter(|custom_structure| custom_structure.id != *id)
            .map(|custom_structure| CustomStructure {
                id: custom_structure.id.clone(),
                name: custom_structure.name.clone(),
                structures: custom_structure.structures.clone(),
            })
            .collect::<Vec<CustomStructure>>();

        *all_custom_structures = updated_structures;

        Ok(())
    }

    pub fn stringify(all_custom_structures: &Vec<CustomStructure>) -> String {
        let mut stringified_custom_structures = String::new();

        for custom_structure in all_custom_structures {
            let stringified_structures = Structure::stringify(&custom_structure.structures);

            stringified_custom_structures = format!(
                "{}{}{}|{}|{}",
                stringified_custom_structures,
                if stringified_custom_structures.chars().count() > 1 {
                    "#"
                } else {
                    ""
                },
                custom_structure.id,
                custom_structure.name,
                stringified_structures,
            );
        }

        stringified_custom_structures
    }
}
