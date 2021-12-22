use crate::structures::Structure;
use wasm_bindgen::prelude::*;

pub fn get_structures_from_str(structures: &str) -> Vec<Structure> {
    let mut all_structures = Vec::<Structure>::new();

    for structure in structures.split("%").collect::<Vec<&str>>() {
        let res_tmp_structure = Structure::from_string(structure);
        if let Ok(tmp_structure) = res_tmp_structure {
            all_structures.push(tmp_structure);
        }
    }

    all_structures
}

pub fn turn_structures_to_str(structures: Vec<Structure>) -> String {
    let mut stringified_structures = String::new();

    for structure in structures {
        stringified_structures = format!(
            "{}{}{}",
            stringified_structures,
            if stringified_structures.chars().count() > 1 {
                "%"
            } else {
                ""
            },
            Structure::to_string(structure.clone())
        );
    }

    stringified_structures
}

#[wasm_bindgen]
pub fn structure_exists(structures: &str, id: &str) -> usize {
    let all_structures = get_structures_from_str(structures);

    if Structure::exist(&all_structures, id) {
        return 1;
    }

    0
}

#[wasm_bindgen]
pub fn create_structure(
    structures: &str,
    id: &str,
    name: &str,
    stype_txt: &str,
    default: &str,
    min: usize,
    max: usize,
    encrypted: bool,
    unique: bool,
    regex_pattern: &str,
    array: bool,
) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::create(
        &mut all_structures,
        id,
        name,
        stype_txt,
        default,
        min,
        max,
        encrypted,
        unique,
        regex_pattern,
        array,
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_id(structures: &str, id: &str, new_id: &str) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_id(&mut all_structures, &id.to_string(), new_id);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_name(structures: &str, id: &str, name: &str) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_name(&mut all_structures, &id.to_string(), name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_type(structures: &str, id: &str, stype: &str) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_type(&mut all_structures, &id.to_string(), stype);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_default(structures: &str, id: &str, default: &str) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_default(&mut all_structures, &id.to_string(), default);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_min(structures: &str, id: &str, min: usize) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_min(&mut all_structures, &id.to_string(), min);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_max(structures: &str, id: &str, max: usize) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_max(&mut all_structures, &id.to_string(), max);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_encrypted(structures: &str, id: &str, encrypted: usize) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result =
        Structure::update_encrypted(&mut all_structures, &id.to_string(), !(encrypted == 0));

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_unique(structures: &str, id: &str, unique: usize) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_unique(&mut all_structures, &id.to_string(), !(unique == 0));

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_regex(structures: &str, id: &str, regex: &str) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_regex(&mut all_structures, &id.to_string(), regex);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_structure_array(structures: &str, id: &str, array: usize) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::update_array(&mut all_structures, &id.to_string(), !(array == 0));

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn delete_structure(structures: &str, id: &str) -> String {
    let mut all_structures = get_structures_from_str(structures);
    let result = Structure::delete(&mut all_structures, &id.to_string());

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}
