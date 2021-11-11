use io::remove_file;
use mappings::{fetch_all_mappings, save_all_mappings, get_file_name, Mapping};
use user::{User, fetch_all_users};

mod io;
mod mappings;
mod user;
mod tests;

const MAPPINGS_PATH: &str = "data/mappings.txt";

fn main() {
    initialize();
}

fn initialize() {
    let all_mappings  = initialize_mappings();
    let all_users = initialize_users(&all_mappings);
    println!("{:#?}", all_users);
}

fn initialize_mappings() -> Vec<Mapping> {
    let mut fetched_mappings = fetch_all_mappings(MAPPINGS_PATH);
    let user_mapping = Mapping::create(&mut fetched_mappings, "users", "data/users.txt");
    if let Err(e) = user_mapping {
        println!("{}", e);
    }

    save_all_mappings(&fetched_mappings, MAPPINGS_PATH);
    fetched_mappings
}

fn initialize_users(mappings: &Vec<Mapping>) {
    let all_users_path = get_file_name("users", mappings);
    let mut all_users = Vec::<User>::new();
    
    if let Ok(path) = all_users_path {
        let fetched_users = fetch_all_users(path.clone());
        all_users = fetched_users;
    }

    all_users;
}

fn reset_db(all_mappings: Vec<Mapping>) {
    remove_file(MAPPINGS_PATH.to_string());
    for mapping in all_mappings.iter() {
        remove_file(mapping.get_file_name());
    }
}
