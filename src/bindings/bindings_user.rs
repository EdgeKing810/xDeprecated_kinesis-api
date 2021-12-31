use crate::user::{fetch_all_users, save_all_users, User};
use wasm_bindgen::prelude::*;

pub fn get_users_from_str(users: &str) -> Vec<User> {
    let mut all_users = Vec::<User>::new();

    for user in users.split("\n").collect::<Vec<&str>>() {
        let tmp_user = User::from_string(user);
        all_users.push(tmp_user);
    }

    all_users
}

pub fn turn_users_to_str(users: Vec<User>) -> String {
    let mut stringified_users = String::new();

    for user in users {
        stringified_users = format!(
            "{}{}{}",
            stringified_users,
            if stringified_users.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            User::to_string(user.clone())
        );
    }

    stringified_users
}

#[wasm_bindgen]
pub fn user_exists(users: &str, id: &str) -> usize {
    let all_users = get_users_from_str(users);

    if User::exist(&all_users, id) {
        return 1;
    }

    0
}

#[wasm_bindgen]
pub fn user_exists_username(users: &str, username: &str) -> usize {
    let all_users = get_users_from_str(users);

    if User::exist_username(&all_users, username) {
        return 1;
    }

    0
}

#[wasm_bindgen]
pub fn user_exists_email(users: &str, email: &str) -> usize {
    let all_users = get_users_from_str(users);

    if User::exist_email(&all_users, email) {
        return 1;
    }

    0
}

/*#[wasm_bindgen]
pub fn register_user(
    users: &str,
    first_name: &str,
    last_name: &str,
    username: &str,
    email: &str,
    password: &str,
    role_numeric: u32,
) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::register(
        &mut all_users,
        first_name,
        last_name,
        username,
        email,
        password,
        role_numeric,
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}*/

#[wasm_bindgen]
pub fn login_user(users: &str, auth: &str, password: &str, encryption_key: &str) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::login(&mut all_users, auth, password, &encryption_key.to_string());

    if let Err(e) = result {
        return e;
    }

    User::to_string(result.unwrap())
}

#[wasm_bindgen]
pub fn update_user_name(users: &str, id: &str, first_name: &str, last_name: &str) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::update_name(&mut all_users, &id.to_string(), first_name, last_name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_user_username(users: &str, id: &str, username: &str) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::update_username(&mut all_users, &id.to_string(), username);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_user_email(users: &str, id: &str, email: &str) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::update_email(&mut all_users, &id.to_string(), email);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_user_password(users: &str, id: &str, password: &str, encryption_key: &str) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::update_password(
        &mut all_users,
        &id.to_string(),
        password,
        &encryption_key.to_string(),
    );

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_user_role(users: &str, id: &str, role_numeric: u32) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::update_role(&mut all_users, &id.to_string(), role_numeric);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn delete_user(users: &str, id: &str) -> String {
    let mut all_users = get_users_from_str(users);
    let result = User::delete(&mut all_users, &id.to_string());

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn fetch_users(path: &str, encryption_key: &str) -> String {
    let all_users = fetch_all_users(String::from(path), &encryption_key.to_string());

    let mut stringified_users = String::new();

    for user in all_users {
        stringified_users = format!(
            "{}{}{}",
            stringified_users,
            if stringified_users.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            User::to_string(user.clone())
        );
    }

    stringified_users
}

#[wasm_bindgen]
pub fn save_users(users: &str, path: &str, encryption_key: &str) {
    let all_users = get_users_from_str(users);

    save_all_users(&all_users, path.to_string(), &encryption_key.to_string());
}
