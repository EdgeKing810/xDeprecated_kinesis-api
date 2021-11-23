#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;

use config::{fetch_all_configs, save_all_configs, Config};
use encryption::{fetch_encryption_key, save_encryption_key, EncryptionKey};
use io::remove_file;
use mappings::{fetch_all_mappings, get_file_name, save_all_mappings, Mapping};
use project::{fetch_all_projects, save_all_projects, Project};
use user::{fetch_all_users, User};

mod config;
mod encryption;
mod io;
mod mappings;
mod project;
mod tests;
mod user;

const MAPPINGS_PATH: &str = "data/mappings.txt";

fn main() {
    initialize();
}

fn initialize() {
    let all_mappings = initialize_mappings();
    let mut all_users: Vec<User> = initialize_users(&all_mappings);
    let mut all_projects: Vec<Project> = initialize_projects(&all_mappings);
    let _all_configs: Vec<Config> = initialize_configs(&all_mappings);

    let tmp_password = "Test123*";
    let init_encryption = initialize_encryption_key(&all_mappings, tmp_password);
    match init_encryption {
        Err(e) => println!("Error: {}", e),
        Ok(key) => println!("Encryption Key: {}", key),
    }

    println!(
        "{:#?}",
        User::login(&mut all_users, "EdgeKing810", "Test123*")
    );

    let create_project = Project::create(
        &mut all_projects,
        "konnect",
        "Konnect - Social Media",
        "A next-gen social media.",
        "/api/v2/konnect",
    );
    if let Err(e) = create_project {
        println!("{}", e);
    }

    save_all_projects(&all_projects, "data/projects.txt");

    println!("Projects: {:#?}", all_projects);
}

fn initialize_mappings() -> Vec<Mapping> {
    let mut fetched_mappings = fetch_all_mappings(MAPPINGS_PATH);

    let user_mapping = Mapping::create(&mut fetched_mappings, "users", "data/users.txt");
    if let Err(e) = user_mapping {
        println!("{}", e);
    }

    let project_mapping = Mapping::create(&mut fetched_mappings, "projects", "data/projects.txt");
    if let Err(e) = project_mapping {
        println!("{}", e);
    }

    let config_mapping = Mapping::create(&mut fetched_mappings, "configs", "data/configs.txt");
    if let Err(e) = config_mapping {
        println!("{}", e);
    }

    let encryption_key_mapping = Mapping::create(
        &mut fetched_mappings,
        "encryption_key",
        "data/encryption_key.txt",
    );
    if let Err(e) = encryption_key_mapping {
        println!("{}", e);
    }

    save_all_mappings(&fetched_mappings, MAPPINGS_PATH);
    fetched_mappings
}

fn initialize_users(mappings: &Vec<Mapping>) -> Vec<User> {
    let all_users_path = get_file_name("users", mappings);
    let mut all_users = Vec::<User>::new();

    if let Ok(path) = all_users_path {
        let fetched_users = fetch_all_users(path.clone());
        all_users = fetched_users;
    }

    all_users
}

fn initialize_projects(mappings: &Vec<Mapping>) -> Vec<Project> {
    let all_projects_path = get_file_name("projects", mappings);
    let mut all_projects = Vec::<Project>::new();

    if let Ok(path) = all_projects_path {
        let fetched_projects = fetch_all_projects(path.clone());
        all_projects = fetched_projects;
    }

    all_projects
}

fn initialize_configs(mappings: &Vec<Mapping>) -> Vec<Config> {
    let all_configs_path = get_file_name("configs", mappings);
    let mut all_configs = Vec::<Config>::new();
    let mut final_path = String::new();

    if let Ok(path) = all_configs_path {
        let fetched_configs = fetch_all_configs(path.clone());
        all_configs = fetched_configs;
        final_path = path;
    }

    let config_keys_template: Vec<&str> = vec![
        "ENV",
        "API_URL",
        "API_PORT",
        "API_PRE",
        "WS_PORT",
        "MONGO_URI",
        "DB_NAME",
        "JWT_EXPIRE",
        "SMTP_USERNAME",
        "SMTP_PASSWORD",
        "SMTP_HOST",
        "SMTP_PORT",
        "TOKEN_KEY",
        "ENCRYPT_KEY",
        "MISC_KEY",
        "USE_REDIS",
        "REDIS_HOST",
        "REDIS_PORT",
        "UPLOAD_SIZE",
        "SHOULD_INITIALIZE",
        "CORS_WHITELIST",
    ];

    for key in config_keys_template {
        let mut found = false;
        for config in all_configs.iter() {
            if key == config.name {
                found = true;
                break;
            }
        }

        if found {
            continue;
        }

        let create_config = Config::create(&mut all_configs, key, "_empty");
        if let Err(e) = create_config {
            println!("{}", e);
        }
    }
    save_all_configs(&all_configs, &*final_path);

    all_configs
}

fn initialize_encryption_key(mappings: &Vec<Mapping>, password: &str) -> Result<String, String> {
    let encryption_key_path = get_file_name("encryption_key", mappings);
    let mut encryption_key_final_path: String = String::new();
    let mut encryption_key: Result<String, String> =
        Err(String::from("Error: Encryption Key not found"));

    if let Ok(path) = encryption_key_path {
        encryption_key = fetch_encryption_key(path.clone(), password);
        encryption_key_final_path = path;
    }

    if let Err(_) = encryption_key {
        // Encryption key most likely doesn't exist yet
        let new_encryption_key = EncryptionKey::generate(20);
        let saved_encryption_key = save_encryption_key(
            new_encryption_key.0.clone(),
            password,
            &*encryption_key_final_path,
        );

        if let Err(f) = saved_encryption_key {
            return Err(String::from(f));
        }

        return Ok(new_encryption_key.0);
    }

    Ok(encryption_key.unwrap())
}

fn reset_db(all_mappings: Vec<Mapping>) {
    remove_file(MAPPINGS_PATH.to_string());
    for mapping in all_mappings.iter() {
        remove_file(mapping.get_file_name());
    }
}
