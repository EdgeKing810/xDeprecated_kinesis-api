use crate::{
    bindings_custom_structures::get_custom_structures_from_str,
    bindings_structures::get_structures_from_str,
    collection::{fetch_all_collections, save_all_collections, Collection},
    custom_structures::CustomStructure,
    structures::Structure,
};
use wasm_bindgen::prelude::*;

pub fn get_collections_from_str(collections: &str) -> Vec<Collection> {
    let mut all_collections = Vec::<Collection>::new();

    for collection in collections.split("\n").collect::<Vec<&str>>() {
        Collection::from_string(&mut all_collections, collection);
    }

    all_collections
}

pub fn turn_collections_to_str(collections: Vec<Collection>) -> String {
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

    stringified_collections
}

#[wasm_bindgen]
pub fn collection_exists(collections: &str, id: &str) -> usize {
    let all_collections = get_collections_from_str(collections);

    if Collection::exist(&all_collections, id) {
        return 1;
    }

    0
}

#[wasm_bindgen]
pub fn create_collection(
    collections: &str,
    id: &str,
    project_id: &str,
    name: &str,
    description: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let result = Collection::create(&mut all_collections, id, project_id, name, description);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_id(collections: &str, id: &str, new_id: &str) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let result = Collection::update_id(&mut all_collections, &id.to_string(), new_id);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_project_id(collections: &str, id: &str, project_id: &str) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let result = Collection::update_project_id(&mut all_collections, &id.to_string(), project_id);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_name(collections: &str, id: &str, name: &str) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let result = Collection::update_name(&mut all_collections, &id.to_string(), name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_description(collections: &str, id: &str, description: &str) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let result = Collection::update_description(&mut all_collections, &id.to_string(), description);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_add_structure(collections: &str, id: &str, structure_str: &str) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let structure_res = Structure::from_string(structure_str);

    if let Err(e) = structure_res {
        return e;
    }

    let result = Collection::add_structure(
        &mut all_collections,
        &id.to_string(),
        structure_res.unwrap(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_update_structure(
    collections: &str,
    id: &str,
    structure_str: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let structure_res = Structure::from_string(structure_str);

    if let Err(e) = structure_res {
        return e;
    }

    let result = Collection::update_structure(
        &mut all_collections,
        &id.to_string(),
        structure_res.unwrap(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_add_custom_structure(
    collections: &str,
    id: &str,
    custom_structure_str: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let custom_structure_res = CustomStructure::from_string(custom_structure_str);

    if let Err(e) = custom_structure_res {
        return e;
    }

    let result = Collection::add_custom_structure(
        &mut all_collections,
        &id.to_string(),
        custom_structure_res.unwrap(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_update_custom_structure(
    collections: &str,
    id: &str,
    custom_structure_str: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let custom_structure_res = CustomStructure::from_string(custom_structure_str);

    if let Err(e) = custom_structure_res {
        return e;
    }

    let result = Collection::update_custom_structure(
        &mut all_collections,
        &id.to_string(),
        custom_structure_res.unwrap(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_set_structures(
    collections: &str,
    id: &str,
    structures_str: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let structures = get_structures_from_str(structures_str);

    let result = Collection::set_structures(&mut all_collections, &id.to_string(), structures);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_set_custom_structures(
    collections: &str,
    id: &str,
    custom_structures_str: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);
    let custom_structures = get_custom_structures_from_str(custom_structures_str);

    let result =
        Collection::set_custom_structures(&mut all_collections, &id.to_string(), custom_structures);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_remove_structure(
    collections: &str,
    id: &str,
    structure_id: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);

    let result = Collection::remove_structure(
        &mut all_collections,
        &id.to_string(),
        &structure_id.to_string(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_collection_remove_custom_structure(
    collections: &str,
    id: &str,
    custom_structure_id: &str,
) -> String {
    let mut all_collections = get_collections_from_str(collections);

    let result = Collection::remove_custom_structure(
        &mut all_collections,
        &id.to_string(),
        &custom_structure_id.to_string(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn delete_collection(collections: &str, id: &str) -> String {
    let mut all_collections = get_collections_from_str(collections);

    let result = Collection::delete(&mut all_collections, &id.to_string());

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn fetch_collections(path: &str, encryption_key: &str) -> String {
    let all_collections = fetch_all_collections(String::from(path), &encryption_key.to_string());

    let mut stringified_collections = String::new();

    for collection in all_collections {
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

    stringified_collections
}

#[wasm_bindgen]
pub fn save_collections(collections: &str, path: &str, encryption_key: &str) {
    let all_collections = get_collections_from_str(collections);

    save_all_collections(
        &all_collections,
        path.to_string(),
        &encryption_key.to_string(),
    );
}
