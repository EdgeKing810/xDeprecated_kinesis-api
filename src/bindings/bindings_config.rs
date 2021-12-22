use crate::config::{fetch_all_configs, save_all_configs, Config};
use wasm_bindgen::prelude::*;

pub fn get_configs_from_str(configs: &str) -> Vec<Config> {
    let mut all_configs = Vec::<Config>::new();

    for config in configs.split("\n").collect::<Vec<&str>>() {
        let tmp_config = Config::from_string(config);
        all_configs.push(tmp_config);
    }

    all_configs
}

pub fn turn_configs_to_str(configs: Vec<Config>) -> String {
    let mut stringified_configs = String::new();

    for config in configs {
        stringified_configs = format!(
            "{}{}{}",
            stringified_configs,
            if stringified_configs.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Config::to_string(config.clone())
        );
    }

    stringified_configs
}

#[wasm_bindgen]
pub fn config_exists(configs: &str, name: &str) -> usize {
    let all_configs = get_configs_from_str(configs);

    if Config::exist(&all_configs, name) {
        return 1;
    }

    0
}

#[wasm_bindgen]
pub fn create_config(
    configs: &str,
    name: &str,
    value: &str,
) -> String {
    let mut all_configs = get_configs_from_str(configs);
    let result = Config::create(&mut all_configs, name, value);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_config_value(
    configs: &str,
    name: &str,
    value: &str,
) -> String {
    let mut all_configs = get_configs_from_str(configs);
    let result = Config::update_value(&mut all_configs, name, value);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn delete_config(
    configs: &str,
    name: &str,
) -> String {
    let mut all_configs = get_configs_from_str(configs);
    let result = Config::delete(&mut all_configs, name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn fetch_configs(path: &str, encryption_key: &str) -> String {
    let all_configs = fetch_all_configs(String::from(path), &encryption_key.to_string());

    let mut stringified_configs = String::new();

    for config in all_configs {
        stringified_configs = format!(
            "{}{}{}",
            stringified_configs,
            if stringified_configs.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Config::to_string(config.clone())
        );
    }

    stringified_configs
}

#[wasm_bindgen]
pub fn save_configs(configs: &str, path: &str, encryption_key: &str) {
    let all_configs = get_configs_from_str(configs);

    save_all_configs(
        &all_configs,
        String::from(path),
        &encryption_key.to_string(),
    );
}
