use crate::{
    bindings_structures::get_structures_from_str, custom_structures::CustomStructure,
    structures::Structure,
};
use wasm_bindgen::prelude::*;

pub fn get_custom_structures_from_str(custom_structures: &str) -> Vec<CustomStructure> {
    let mut all_custom_structures = Vec::<CustomStructure>::new();

    for custom_structure in custom_structures.split("#").collect::<Vec<&str>>() {
        let res_tmp_custom_structure = CustomStructure::from_string(custom_structure);
        if let Ok(tmp_custom_structure) = res_tmp_custom_structure {
            all_custom_structures.push(tmp_custom_structure);
        }
    }

    all_custom_structures
}

pub fn turn_custom_structures_to_str(custom_structures: Vec<CustomStructure>) -> String {
    let mut stringified_custom_structures = String::new();

    for custom_structure in custom_structures {
        stringified_custom_structures = format!(
            "{}{}{}",
            stringified_custom_structures,
            if stringified_custom_structures.chars().count() > 1 {
                "#"
            } else {
                ""
            },
            CustomStructure::to_string(custom_structure.clone())
        );
    }

    stringified_custom_structures
}

#[wasm_bindgen]
pub fn custom_structure_exists(custom_structures: &str, id: &str) -> usize {
    let all_custom_structures = get_custom_structures_from_str(custom_structures);

    if CustomStructure::exist(&all_custom_structures, id) {
        return 1;
    }

    0
}

#[wasm_bindgen]
pub fn create_custom_structure(custom_structures: &str, id: &str, name: &str) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);
    let result = CustomStructure::create(&mut all_custom_structures, id, name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_custom_structure_id(custom_structures: &str, id: &str, new_id: &str) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);
    let result = CustomStructure::update_id(&mut all_custom_structures, &id.to_string(), new_id);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_custom_structure_name(custom_structures: &str, id: &str, name: &str) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);
    let result = CustomStructure::update_name(&mut all_custom_structures, &id.to_string(), name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_custom_structure_add_structure(
    custom_structures: &str,
    id: &str,
    structure_str: &str,
) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);
    let structure_res = Structure::from_string(structure_str);

    if let Err(e) = structure_res {
        return e;
    }

    let result = CustomStructure::add_structure(
        &mut all_custom_structures,
        &id.to_string(),
        structure_res.unwrap(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_custom_structure_update_structure(
    custom_structures: &str,
    id: &str,
    structure_str: &str,
) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);
    let structure_res = Structure::from_string(structure_str);

    if let Err(e) = structure_res {
        return e;
    }

    let result = CustomStructure::update_structure(
        &mut all_custom_structures,
        &id.to_string(),
        structure_res.unwrap(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_custom_structure_set_structures(
    custom_structures: &str,
    id: &str,
    structures_str: &str,
) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);
    let structures = get_structures_from_str(structures_str);

    let result =
        CustomStructure::set_structures(&mut all_custom_structures, &id.to_string(), structures);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_custom_structure_remove_structure(
    custom_structures: &str,
    id: &str,
    structure_id: &str,
) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);

    let result = CustomStructure::remove_structure(
        &mut all_custom_structures,
        &id.to_string(),
        &structure_id.to_string(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn delete_custom_structure(custom_structures: &str, id: &str) -> String {
    let mut all_custom_structures = get_custom_structures_from_str(custom_structures);

    let result = CustomStructure::delete(&mut all_custom_structures, &id.to_string());

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}
