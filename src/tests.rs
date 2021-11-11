#[cfg(test)]
use crate::{
    io::remove_file,
    mappings::{fetch_all_mappings, Mapping},
};

#[test]
fn test_mappings() {
    let file_name: &str = "data/mappings_test.txt";
    remove_file(file_name.to_string());

    let mut all_mappings = fetch_all_mappings(file_name);
    println!("{:#?}", all_mappings);

    let user_mapping = Mapping::create(&mut all_mappings, "users_test", "users.txt");
    assert_eq!(user_mapping, Ok(()));

    let user_mapping2 = Mapping::create(&mut all_mappings, "users_test", "users.txt");
    assert_eq!(
        user_mapping2,
        Err(String::from(
            "A similar Mapping already exists (users_test)"
        ))
    );

    let weird = Mapping::create(&mut all_mappings, "weird", "weird.txt");
    assert_eq!(weird, Ok(()));

    let weird2 = Mapping::create(&mut all_mappings, "weird2", "weird.");
    assert_eq!(
        weird2,
        Err(String::from(
            "Invalid character (perhaps a symbol?) found in id or file_name (weird2)"
        ))
    );

    let weird3 = Mapping::create(&mut all_mappings, "weird3", "weird");
    assert_eq!(weird3, Ok(()));

    let weird = Mapping::update(&mut all_mappings, "weird", "not_weird.txt");
    assert_eq!(weird, Ok(()));

    let remove_weird3 = Mapping::remove(&mut all_mappings, "weird3");
    assert_eq!(remove_weird3, Ok(()));
}
