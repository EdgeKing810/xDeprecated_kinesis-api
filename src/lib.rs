#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;

use bindings_user::{delete_user, get_users_from_str, login_user};
use user::User;
use wasm_bindgen::prelude::*;

mod bcrypt;
mod collection;
mod config;
mod custom_structures;
mod encryption;
mod io;
mod mappings;
mod project;
mod structures;
mod user;

#[path = "bindings/bindings_mappings.rs"]
mod bindings_mappings;
#[path = "bindings/bindings_project.rs"]
mod bindings_project;
#[path = "bindings/bindings_user.rs"]
mod bindings_user;

const MAPPINGS_PATH: &str = "data/mappings.txt";

#[wasm_bindgen]
pub fn get_mappings_path() -> String {
    MAPPINGS_PATH.to_string()
}

#[wasm_bindgen]
pub fn moggt(users: &str, auth: &str, password: &str) -> String {
    login_user(users, auth, password)
}

#[wasm_bindgen]
pub fn login_ggt_user(users: &str, auth: &str, password: &str) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::login(&mut all_users, auth, password);

    if let Err(e) = result {
        return e;
    }

    User::to_string(result.unwrap())
}

#[wasm_bindgen]
pub fn delete_ggt_user(users: &str, id: &str) -> String {
    delete_user(users, id)
}
