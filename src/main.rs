use io::remove_file;
use mappings::{fetch_all_mappings, save_all_mappings, Mapping};

mod io;
mod mappings;
mod tests;

const MAPPINGS_PATH: &str = "data/mappings.txt";

fn main() {
    initialize();
}

fn initialize() {
    initialize_mappings();
}

fn initialize_mappings() {
    let mut all_mappings = fetch_all_mappings(MAPPINGS_PATH);
    let user_mapping = Mapping::create(&mut all_mappings, "users", "data/users.txt");
    if let Err(e) = user_mapping {
        println!("{}", e);
    }

    save_all_mappings(all_mappings, MAPPINGS_PATH);
}

fn reset_db(all_mappings: Vec<Mapping>) {
    remove_file(MAPPINGS_PATH.to_string());
    for mapping in all_mappings.iter() {
        remove_file(mapping.get_file_name());
    }
}
