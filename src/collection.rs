use crate::custom_structures::CustomStructure;
use crate::io::{fetch_file, save_file};
use crate::structures::{try_add_structure, Structure};
// use crate::encryption::{EncryptionKey};

#[derive(Default, Debug, Clone)]
pub struct Collection {
    id: String,
    project_id: String,
    name: String,
    description: String,
    structures: Vec<Structure>,
    custom_structures: Vec<CustomStructure>,
}

impl Collection {
    pub fn create(
        collections: &mut Vec<Collection>,
        id: &str,
        project_id: &str,
        name: &str,
        description: &str,
    ) -> Result<(), String> {
        // if Self::exist(collections, id) {
        //     let new_id = EncryptionKey::generate_uuid();
        //     return Self::create(
        //         collections,
        //         &*new_id.to_string(),
        //         project_id,
        //         name,
        //         description,
        //     );
        // }
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: String = String::new();

        let new_collection = Collection {
            id: tmp_id.clone(),
            project_id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            structures: vec![],
            custom_structures: vec![],
        };
        collections.push(new_collection);

        let id_update = Self::update_id(collections, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("Error: {}", e);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let project_id_update = Self::update_project_id(collections, &new_id, project_id);
            if let Err(e) = project_id_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let name_update = Self::update_name(collections, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let description_update = Self::update_description(collections, &new_id, description);
            if let Err(e) = description_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if has_error {
            let delete_collection = Self::delete(collections, &new_id);
            if let Err(e) = delete_collection {
                println!("Error: {}", e);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_collections: &Vec<Collection>, id: &str) -> bool {
        let mut found = false;
        for collection in all_collections.iter() {
            if collection.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_id(
        all_collections: &mut Vec<Collection>,
        id: &String,
        new_id: &str,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *new_id {
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
            return Err(String::from("Error: id does not contain enough characters"));
        } else if String::from(new_id.trim()).len() > 100 {
            return Err(String::from("Error: new_id contains too many characters"));
        }

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                collection.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn update_project_id(
        all_collections: &mut Vec<Collection>,
        id: &String,
        project_id: &str,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        if !String::from(project_id)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(String::from(
                "Error: project_id contains an invalid character",
            ));
        }

        if String::from(project_id.trim()).len() < 1 {
            return Err(String::from(
                "Error: project_id does not contain enough characters",
            ));
        } else if String::from(project_id.trim()).len() > 100 {
            return Err(String::from(
                "Error: project_id contains too many characters",
            ));
        }

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                collection.project_id = project_id.trim().to_string();
                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn update_name(
        all_collections: &mut Vec<Collection>,
        id: &String,
        name: &str,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

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

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                collection.name = name.trim().to_string();
                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn update_description(
        all_collections: &mut Vec<Collection>,
        id: &String,
        description: &str,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        if !String::from(description)
            .chars()
            .all(|c| c != ';' && c != '@' && c != '>' && c != '#')
        {
            return Err(String::from(
                "Error: description contains an invalid character",
            ));
        }

        if String::from(description.trim()).len() < 1 {
            return Err(String::from(
                "Error: description does not contain enough characters",
            ));
        } else if String::from(description.trim()).len() > 400 {
            return Err(String::from(
                "Error: description contains too many characters",
            ));
        }

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                collection.description = description.trim().to_string();
                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn add_structure(
        all_collections: &mut Vec<Collection>,
        id: &String,
        structure: Structure,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());

                let mut current_structures = collection.structures.clone();
                current_structures.push(structure);
                collection.structures = current_structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn update_structure(
        all_collections: &mut Vec<Collection>,
        id: &String,
        structure: Structure,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                let mut found_structure = false;

                let mut current_structures = collection.structures.clone();

                for current_structure in current_structures.iter_mut() {
                    if current_structure.id == structure.id {
                        *current_structure = structure.clone();
                        found_structure = true;
                    }
                }

                if !found_structure {
                    current_structures.push(structure);
                }

                collection.structures = current_structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn add_custom_structure(
        all_collections: &mut Vec<Collection>,
        id: &String,
        custom_structure: CustomStructure,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());

                let mut current_custom_structures = collection.custom_structures.clone();
                current_custom_structures.push(custom_structure);
                collection.custom_structures = current_custom_structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn update_custom_structure(
        all_collections: &mut Vec<Collection>,
        id: &String,
        custom_structure: CustomStructure,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                let mut found_structure = false;

                let mut current_custom_structures = collection.custom_structures.clone();

                for current_custom_structure in current_custom_structures.iter_mut() {
                    if current_custom_structure.id == custom_structure.id {
                        *current_custom_structure = custom_structure.clone();
                        found_structure = true;
                    }
                }

                if !found_structure {
                    current_custom_structures.push(custom_structure);
                }

                collection.custom_structures = current_custom_structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn set_structures(
        all_collections: &mut Vec<Collection>,
        id: &String,
        structures: Vec<Structure>,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                collection.structures = structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn set_custom_structures(
        all_collections: &mut Vec<Collection>,
        id: &String,
        custom_structures: Vec<CustomStructure>,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());
                collection.custom_structures = custom_structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn remove_structure(
        all_collections: &mut Vec<Collection>,
        id: &String,
        structure_id: &String,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());

                let mut current_structures = collection.structures.clone();
                let result_delete_structure =
                    Structure::delete(&mut current_structures, structure_id);
                if let Err(e) = result_delete_structure {
                    return Err(e);
                }
                collection.structures = current_structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn remove_custom_structure(
        all_collections: &mut Vec<Collection>,
        id: &String,
        custom_structure_id: &String,
    ) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == *id {
                found_collection = Some(collection.clone());

                let mut current_custom_structures = collection.custom_structures.clone();
                let result_delete_custom_structure =
                    CustomStructure::delete(&mut current_custom_structures, custom_structure_id);
                if let Err(e) = result_delete_custom_structure {
                    return Err(e);
                }
                collection.custom_structures = current_custom_structures;

                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        Ok(())
    }

    pub fn delete(all_collections: &mut Vec<Collection>, id: &String) -> Result<(), String> {
        let mut found_collection: Option<Collection> = None;

        for collection in all_collections.iter_mut() {
            if collection.id == id.to_string() {
                found_collection = Some(collection.clone());
                break;
            }
        }

        if let None = found_collection {
            return Err(String::from("Error: Collection not found"));
        }

        let updated_collections: Vec<Collection> = all_collections
            .iter_mut()
            .filter(|collection| collection.id != *id)
            .map(|collection| Collection {
                id: collection.id.clone(),
                project_id: collection.project_id.clone(),
                name: collection.name.clone(),
                description: collection.description.clone(),
                structures: collection.structures.clone(),
                custom_structures: collection.custom_structures.clone(),
            })
            .collect::<Vec<Collection>>();

        *all_collections = updated_collections;

        Ok(())
    }

    pub fn to_string(collection: Collection) -> String {
        let stringified_structures = Structure::stringify(&collection.structures);

        let stringified_custom_structures =
            CustomStructure::stringify(&collection.custom_structures);

        format!(
            "{};{};{};{}>{}>{}",
            collection.id,
            collection.project_id,
            collection.name,
            collection.description,
            stringified_structures,
            stringified_custom_structures
        )
    }

    pub fn from_string(mut all_collections: &mut Vec<Collection>, collection_str: &str) -> String {
        let current_collection = collection_str.split(";").collect::<Vec<&str>>();

        let collection_id = current_collection[0];
        let create_collection = Collection::create(
            &mut all_collections,
            current_collection[0],
            current_collection[1],
            current_collection[2],
            current_collection[3].split(">").collect::<Vec<&str>>()[0],
        );
        if let Err(e) = create_collection {
            return e;
        }

        let current_structures = collection_str.split(">").collect::<Vec<&str>>()[1];
        let individual_structures = current_structures.split("%").collect::<Vec<&str>>();
        let mut final_structures: Vec<Structure> = vec![];
        for structure in individual_structures {
            let current_structure = structure.split("|").collect::<Vec<&str>>();

            if !try_add_structure(&current_structure, &mut final_structures) {
                continue;
            }
        }

        let current_custom_structures = collection_str.split(">").collect::<Vec<&str>>()[2];
        let individual_custom_structures =
            current_custom_structures.split("#").collect::<Vec<&str>>();
        let mut final_custom_structures: Vec<CustomStructure> = vec![];
        for custom_structure in individual_custom_structures {
            let current_custom_structure = custom_structure.split("|").collect::<Vec<&str>>();

            if current_custom_structure.len() <= 1 {
                break;
            }

            let create_custom_structure = CustomStructure::create(
                &mut final_custom_structures,
                current_custom_structure[0],
                current_custom_structure[1],
            );
            if let Err(e) = create_custom_structure {
                return e;
            }

            let current_structures = current_custom_structure[2..].join("|");
            let individual_structures = current_structures.split("%").collect::<Vec<&str>>();
            let mut final_structures_custom: Vec<Structure> = vec![];
            for structure in individual_structures {
                let current_structure = structure.split("|").collect::<Vec<&str>>();

                if !try_add_structure(&current_structure, &mut final_structures_custom) {
                    continue;
                }
            }

            let custom_set_structures = CustomStructure::set_structures(
                &mut final_custom_structures,
                &current_custom_structure[0].to_string(),
                final_structures_custom,
            );
            if let Err(e) = custom_set_structures {
                return e;
            }
        }

        let set_structures = Collection::set_structures(
            &mut all_collections,
            &collection_id.to_string(),
            final_structures,
        );
        if let Err(e) = set_structures {
            return e;
        }

        let set_custom_structures = Collection::set_custom_structures(
            &mut all_collections,
            &collection_id.to_string(),
            final_custom_structures,
        );
        if let Err(e) = set_custom_structures {
            return e;
        }

        String::new()
    }
}

pub fn fetch_all_collections(path: String, encryption_key: &String) -> Vec<Collection> {
    let all_collections_raw = fetch_file(path.clone(), encryption_key);

    let individual_collections = all_collections_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_collections: Vec<Collection> = Vec::<Collection>::new();

    for collection in individual_collections {
        Collection::from_string(&mut final_collections, collection);
    }

    final_collections
}

pub fn save_all_collections(collections: &Vec<Collection>, path: String, encryption_key: &String) {
    let mut stringified_collections = String::new();

    for collection in collections {
        stringified_collections = format!(
            "{}{}{}",
            stringified_collections,
            if stringified_collections.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Collection::to_string(collection.clone())
        );
    }

    save_file(path, stringified_collections, encryption_key);
    println!("Collections saved!");
}
