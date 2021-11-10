use mappings::{fetch_all_mappings, save_all_mappings, Mapping};

mod io;
mod mappings;

fn main() {
    initialize();
}

fn initialize() {
    initialize_mappings();
}

fn initialize_mappings() {
    let mut all_mappings = fetch_all_mappings();
    println!("{:#?}", all_mappings);

    let user_mapping = Mapping::create(&mut all_mappings, "users", "users.txt");
    if let Err(e) = user_mapping {
        println!("{}", e);
    }

    let user_mapping2 = Mapping::create(&mut all_mappings, "users", "users.txt");
    if let Err(e) = user_mapping2 {
        println!("{}", e);
    }

    let weird = Mapping::create(&mut all_mappings, "weird", "weird.txt");
    if let Err(e) = weird {
        println!("{}", e);
    }

    let weird2 = Mapping::create(&mut all_mappings, "weird2", "weird.");
    if let Err(e) = weird2 {
        println!("{}", e);
    }

    let weird3 = Mapping::create(&mut all_mappings, "weird3", "weird");
    if let Err(e) = weird3 {
        println!("{}", e);
    }

    let weird = Mapping::update(&mut all_mappings, "weird", "not_weird.txt");
    if let Err(e) = weird {
        println!("{}", e);
    }

    let remove_weird3 = Mapping::remove(&mut all_mappings, "weird3");
    if let Err(e) = remove_weird3 {
        println!("{}", e);
    }

    save_all_mappings(all_mappings);
}
