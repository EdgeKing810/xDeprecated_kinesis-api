#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;

use config::{fetch_all_configs, save_all_configs, Config};
use encryption::{fetch_encryption_key, save_encryption_key, EncryptionKey};
use io::remove_file;
use mappings::{fetch_all_mappings, get_file_name, save_all_mappings, Mapping};
use project::{fetch_all_projects, save_all_projects, Project};
use user::{fetch_all_users, save_all_users, User};

mod collection;
mod config;
mod custom_structures;
mod encryption;
mod io;
mod mappings;
mod project;
mod structures;
mod tests;
mod user;

const MAPPINGS_PATH: &str = "data/mappings.txt";
const TMP_PASSWORD: &str = "Test123*";

fn main() {
    initialize();
}

fn initialize() {
    let all_mappings = initialize_mappings();
    let all_users: Vec<User> = initialize_users(&all_mappings);
    let all_projects: Vec<Project> = initialize_projects(&all_mappings);
    let _all_configs: Vec<Config> = initialize_configs(&all_mappings);

    println!("{:#?}", User::login(&all_users, "EdgeKing810", "Test123*"));

    println!("Projects: {:#?}", all_projects);
}

fn initialize_mappings() -> Vec<Mapping> {
    let mut fetched_mappings = fetch_all_mappings(MAPPINGS_PATH, &String::new());

    if !Mapping::exist(&fetched_mappings, "users") {
        let user_mapping = Mapping::create(&mut fetched_mappings, "users", "data/users.txt");
        if let Err(e) = user_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "projects") {
        let project_mapping =
            Mapping::create(&mut fetched_mappings, "projects", "data/projects.txt");
        if let Err(e) = project_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "configs") {
        let config_mapping = Mapping::create(&mut fetched_mappings, "configs", "data/configs.txt");
        if let Err(e) = config_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "encryption_key") {
        let encryption_key_mapping = Mapping::create(
            &mut fetched_mappings,
            "encryption_key",
            "data/encryption_key.txt",
        );
        if let Err(e) = encryption_key_mapping {
            println!("{}", e);
        }
    }

    save_all_mappings(&fetched_mappings, MAPPINGS_PATH, &String::from(""));
    fetched_mappings
}

fn initialize_users(mappings: &Vec<Mapping>) -> Vec<User> {
    let all_users_path = get_file_name("users", mappings);
    let mut all_users = Vec::<User>::new();

    if let Err(e) = all_users_path {
        println!("{}", e);
        return all_users;
    }

    all_users = fetch_all_users(
        all_users_path.clone().unwrap(),
        &get_encryption_key(&mappings),
    );

    if !User::exist_username(&all_users, "EdgeKing810") {
        let create_user = User::create(
            &mut all_users,
            "Kishan",
            "Takoordyal",
            "EdgeKing810",
            "kishan@konnect.dev",
            "Test123*",
            0,
        );
        if let Err(e) = create_user {
            println!("{}", e);
        }
    }

    save_all_users(
        &all_users,
        all_users_path.unwrap(),
        &get_encryption_key(&mappings),
    );

    all_users
}

fn initialize_projects(mappings: &Vec<Mapping>) -> Vec<Project> {
    let all_projects_path = get_file_name("projects", mappings);
    let mut all_projects = Vec::<Project>::new();

    if let Err(e) = all_projects_path {
        println!("{}", e);
        return all_projects;
    }

    all_projects = fetch_all_projects(
        all_projects_path.clone().unwrap(),
        &get_encryption_key(&mappings),
    );

    if !Project::exist(&all_projects, "konnect") {
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
    }

    save_all_projects(
        &all_projects,
        all_projects_path.unwrap(),
        &get_encryption_key(&mappings),
    );

    all_projects
}

fn initialize_configs(mappings: &Vec<Mapping>) -> Vec<Config> {
    let all_configs_path = get_file_name("configs", mappings);
    let mut all_configs = Vec::<Config>::new();

    if let Err(e) = all_configs_path {
        println!("{}", e);
        return all_configs;
    }

    all_configs = fetch_all_configs(
        all_configs_path.clone().unwrap(),
        &get_encryption_key(&mappings),
    );

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
        if !Config::exist(&all_configs, key) {
            let create_config = Config::create(&mut all_configs, key, "_empty");
            if let Err(e) = create_config {
                println!("{}", e);
            }
        }
    }
    save_all_configs(
        &all_configs,
        all_configs_path.unwrap(),
        &get_encryption_key(&mappings),
    );

    all_configs
}

fn initialize_encryption_key(mappings: &Vec<Mapping>, password: &str) -> Result<String, String> {
    let encryption_key_path = get_file_name("encryption_key", mappings);
    let encryption_key: Result<String, String>;

    if let Err(e) = encryption_key_path {
        return Err(e);
    }

    encryption_key = fetch_encryption_key(encryption_key_path.clone().unwrap(), password);

    if let Err(_) = encryption_key {
        // Encryption key most likely doesn't exist yet
        let new_encryption_key = EncryptionKey::generate(20);
        let saved_encryption_key = save_encryption_key(
            new_encryption_key.0.clone(),
            password,
            &*encryption_key_path.unwrap(),
        );

        if let Err(f) = saved_encryption_key {
            return Err(String::from(f));
        }

        println!("Encryption Key Saved!");

        return Ok(new_encryption_key.0);
    }

    Ok(encryption_key.unwrap())
}

fn get_encryption_key(all_mappings: &Vec<Mapping>) -> String {
    let init_encryption = initialize_encryption_key(&all_mappings, TMP_PASSWORD);

    if let Err(e) = init_encryption {
        println!("Error: {}", e);
        return String::new();
    }

    init_encryption.unwrap()
}

fn reset_db(all_mappings: Vec<Mapping>) {
    remove_file(MAPPINGS_PATH.to_string());
    for mapping in all_mappings.iter() {
        remove_file(mapping.get_file_name());
    }
}
