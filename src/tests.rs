#![allow(unused_assignments)]
#[cfg(test)]
use crate::{
    collection::{fetch_all_collections, save_all_collections, Collection},
    config::{fetch_all_configs, save_all_configs, Config},
    custom_structures::CustomStructure,
    encryption::{fetch_encryption_key, save_encryption_key, EncryptionKey},
    io::remove_file,
    mappings::{fetch_all_mappings, save_all_mappings, Mapping},
    project::{fetch_all_projects, save_all_projects, Project},
    structures::Structure,
    user::{fetch_all_users, save_all_users, User},
};

#[test]
fn test_mappings() {
    let file_name: &str = "data/mappings_test.txt";
    remove_file(file_name.to_string());

    let mut all_mappings = fetch_all_mappings(file_name, &String::new());
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

    save_all_mappings(&all_mappings, file_name, &String::new());
}

#[test]
fn test_users() {
    let file_name: &str = "data/users_test.txt";
    remove_file(file_name.to_string());

    let mut all_users = fetch_all_users(file_name.to_string(), &String::new());
    println!("{:#?}", all_users);

    let test_user = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test",
        "test@test.com",
        "Test123*",
        0,
    );
    assert_eq!(test_user, Ok(()));

    let test_user2 = User::create(
        &mut all_users,
        "Test2",
        "Tester",
        "test2",
        "test@test2.com",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: first_name contains an invalid character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester2",
        "test2",
        "test@test2.com",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: last_name contains an invalid character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test",
        "test@test2.com",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from("Error: username already taken"))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test.com",
        "Test123*",
        0,
    );
    assert_eq!(test_user2, Err(String::from("Error: email already taken")));

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@@test2.teeeeeeeeest",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from("Error: Invalid email address"))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2..teeeeeeeeest",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from("Error: Invalid email address"))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "Test",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should be longer than 7 characters"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "testtest",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should contain at least 1 uppercase alphabetic character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "TESTTEST",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should contain at least 1 lowercase alphabetic character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "testTEST",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should contain at least 1 number"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "Test123;",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password contains a forbidden character (;)"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "te_st",
        "test@test2.com",
        "Test123*&^()[]{}*-_",
        0,
    );
    assert_eq!(test_user2, Ok(()));

    let login_test_user2 = User::login(&all_users, "te_st", "Test123*&^()[]{}*-_");

    if let Ok(successful_login) = login_test_user2 {
        let test_user2 = User::update_name(&mut all_users, &successful_login.id, "Test", "Tester");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_username(&mut all_users, &successful_login.id, "test2");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_email(&mut all_users, &successful_login.id, "test2@test.com");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_password(&mut all_users, &successful_login.id, "Test123*");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_role(&mut all_users, &successful_login.id, 2);
        assert_eq!(test_user2, Ok(()));
    };

    save_all_users(&all_users, String::from(file_name), &String::new());
}

#[test]
fn test_projects() {
    let file_name: &str = "data/projects_test.txt";
    remove_file(file_name.to_string());

    let mut all_projects = fetch_all_projects(file_name.to_string(), &String::new());
    println!("{:#?}", all_projects);

    let test_project = Project::create(
        &mut all_projects,
        "test",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(test_project, Ok(()));

    let test_project2 = Project::create(
        &mut all_projects,
        "test ",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: new_id contains an invalid character"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test *** Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: name contains an invalid character"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects-",
    );
    assert_eq!(
        test_project2,
        Err(String::from(
            "Error: api_path contains an invalid character"
        ))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project.",
        "/api/v1/Projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from(
            "Error: api_path should not contain uppercase alphabetical character(s)"
        ))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: id is already in use"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: api_path is already in use"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project;",
        "/api/v1/projects2",
    );
    assert_eq!(
        test_project2,
        Err(String::from(
            "Error: description contains an invalid character"
        ))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a new test project.",
        "/api/v1/projects2",
    );
    assert_eq!(test_project2, Ok(()));

    let test2_id = String::from("test2");

    let test_project3 = Project::update_name(&mut all_projects, &test2_id, "Test Project 3");
    assert_eq!(test_project3, Ok(()));

    let test_project3 = Project::update_description(
        &mut all_projects,
        &test2_id,
        "This is a new test project (3).",
    );
    assert_eq!(test_project3, Ok(()));

    let test_project3 = Project::update_api_path(&mut all_projects, &test2_id, "/api/v1/projects3");
    assert_eq!(test_project3, Ok(()));

    let test_project3 = Project::update_id(&mut all_projects, &test2_id, "test3");
    assert_eq!(test_project3, Ok(()));

    save_all_projects(&all_projects, String::from(file_name), &String::new());
}

#[test]
fn test_configs() {
    let file_name: &str = "data/configs_test.txt";
    remove_file(file_name.to_string());

    let mut all_configs = fetch_all_configs(file_name.to_string(), &String::new());
    println!("{:#?}", all_configs);

    let test_config = Config::create(&mut all_configs, "TEST", "test");
    assert_eq!(test_config, Ok(()));

    let test_config2 = Config::create(&mut all_configs, "test?", "Test2");
    assert_eq!(
        test_config2,
        Err(String::from("Error: name contains an invalid character"))
    );

    let test_config2 = Config::create(&mut all_configs, "test", "Test2");
    assert_eq!(
        test_config2,
        Err(String::from(
            "Error: A config with that name already exists (TEST)"
        ))
    );

    let test_config2 = Config::create(&mut all_configs, "test2", "Test2|");
    assert_eq!(
        test_config2,
        Err(String::from("Error: value contains an invalid character"))
    );

    let test_config2 = Config::create(&mut all_configs, "test2", "Test2");
    assert_eq!(test_config2, Ok(()));

    let test2_id = "test2";

    let test_config2 = Config::update_value(&mut all_configs, test2_id, "TEST2VAL");
    assert_eq!(test_config2, Ok(()));

    save_all_configs(&all_configs, String::from(file_name), &String::new());
}

#[test]
fn test_encryption() {
    let file_name: &str = "data/encryption_key_test.txt";
    remove_file(file_name.to_string());

    let password: &str = "Test123*";
    let length: usize = 30;

    let encryption_key = fetch_encryption_key(file_name.to_string(), password);
    println!("{:#?}", encryption_key);

    let generated_encryption_key = EncryptionKey::generate(length);
    let encrypted_generated_encryption_key =
        EncryptionKey::encrypt(generated_encryption_key.0.clone(), password);
    let decrypted_generated_encryption_key =
        EncryptionKey::decrypt(encrypted_generated_encryption_key, password);

    assert_eq!(
        decrypted_generated_encryption_key.unwrap().0,
        generated_encryption_key.0
    );

    let saved_encryption_key = save_encryption_key(generated_encryption_key.0, password, file_name);
    if let Err(e) = saved_encryption_key {
        println!("Error: {}", e);
    }
}

#[test]
fn test_correct_collection() {
    let file_name: &str = "data/collection_ok_test.txt";
    remove_file(file_name.to_string());

    let mut all_collections = Vec::<Collection>::new();
    all_collections = fetch_all_collections(file_name.to_string(), &String::new());

    if !Collection::exist(&all_collections, "posts") {
        let create_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        if let Err(e) = create_collection {
            println!("{}", e);
        }

        let mut all_structures = Vec::<Structure>::new();
        Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "cover_image",
            "Cover Image",
            "media",
            "https://test.image.com",
            0,
            200,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "content",
            "Content",
            "richtext",
            "[ Content goes here ]",
            30,
            2000,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "views",
            "Views",
            "number",
            "0",
            0,
            9999,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "comment",
            "Comments",
            "comment",
            "0",
            0,
            9999,
            false,
            false,
            "",
            true,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "published",
            "Published",
            "boolean",
            "false",
            0,
            5,
            false,
            false,
            "",
            true,
        )
        .unwrap();
        Collection::set_structures(&mut all_collections, &"posts".to_string(), all_structures)
            .unwrap();

        let mut all_custom_structures = Vec::<CustomStructure>::new();
        let mut tmp_structures = Vec::<Structure>::new();

        Structure::create(
            &mut tmp_structures,
            "uid",
            "UID",
            "uid",
            "",
            5,
            20,
            false,
            true,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut tmp_structures,
            "value",
            "Value",
            "text",
            "",
            1,
            100,
            false,
            false,
            "",
            false,
        )
        .unwrap();

        CustomStructure::create(&mut all_custom_structures, "comment", "comment").unwrap();
        CustomStructure::set_structures(
            &mut all_custom_structures,
            &"comment".to_string(),
            tmp_structures,
        )
        .unwrap();
        Collection::set_custom_structures(
            &mut all_collections,
            &"posts".to_string(),
            all_custom_structures,
        )
        .unwrap();
    }
    save_all_collections(&all_collections, file_name.to_string(), &String::new());
}

#[test]
fn test_incorrect_collection() {
    let file_name: &str = "data/collection_err_test.txt";
    remove_file(file_name.to_string());

    let mut all_collections = Vec::<Collection>::new();
    all_collections = fetch_all_collections(file_name.to_string(), &String::new());

    if !Collection::exist(&all_collections, "posts") {
        let create_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        if let Err(e) = create_collection {
            println!("{}", e);
        }

        let mut all_structures = Vec::<Structure>::new();
        Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        )
        .unwrap();

        let test_structure = Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        );
        assert_eq!(
            test_structure,
            Err(String::from("Error: id is already in use"))
        );

        let test_structure = Structure::create(
            &mut all_structures,
            "title2=",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        );
        assert_eq!(
            test_structure,
            Err(String::from("Error: new_id contains an invalid character"))
        );

        let test_structure =
            Structure::update_id(&mut all_structures, &"title2".to_string(), "title3");
        assert_eq!(
            test_structure,
            Err(String::from("Error: Structure not found"))
        );

        let test_structure =
            Structure::update_name(&mut all_structures, &"title".to_string(), "Title-");
        assert_eq!(
            test_structure,
            Err(String::from("Error: name contains an invalid character"))
        );

        let test_structure =
            Structure::update_type(&mut all_structures, &"title".to_string(), "test;");
        assert_eq!(
            test_structure,
            Err(String::from(
                "Error: stype_txt contains an invalid character"
            ))
        );

        let test_structure =
            Structure::update_default(&mut all_structures, &"title".to_string(), "test@");
        assert_eq!(
            test_structure,
            Err(String::from(
                "Error: default_val contains an invalid character"
            ))
        );

        let test_structure =
            Structure::update_regex(&mut all_structures, &"title".to_string(), "^;$");
        assert_eq!(
            test_structure,
            Err(String::from(
                "Error: regex_pattern contains an invalid character"
            ))
        );

        Collection::set_structures(&mut all_collections, &"posts".to_string(), all_structures)
            .unwrap();

        let mut all_custom_structures = Vec::<CustomStructure>::new();
        let mut tmp_structures = Vec::<Structure>::new();

        Structure::create(
            &mut tmp_structures,
            "uid",
            "UID",
            "uid",
            "",
            5,
            20,
            false,
            true,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut tmp_structures,
            "value",
            "Value",
            "text",
            "",
            1,
            100,
            false,
            false,
            "",
            false,
        )
        .unwrap();

        CustomStructure::create(&mut all_custom_structures, "comment", "comment").unwrap();
        CustomStructure::set_structures(
            &mut all_custom_structures,
            &"comment".to_string(),
            tmp_structures,
        )
        .unwrap();

        let test_custom_structure =
            CustomStructure::create(&mut all_custom_structures, "comment", "comment");
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: id is already in use"))
        );

        let test_custom_structure = CustomStructure::update_id(
            &mut all_custom_structures,
            &"comment2".to_string(),
            "comment3",
        );
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: Custom Structure not found"))
        );

        let test_custom_structure = CustomStructure::update_id(
            &mut all_custom_structures,
            &"comment".to_string(),
            "comment*",
        );
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: new_id contains an invalid character"))
        );

        let test_custom_structure = CustomStructure::update_name(
            &mut all_custom_structures,
            &"comment".to_string(),
            "comment^^^",
        );
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: name contains an invalid character"))
        );

        Collection::set_custom_structures(
            &mut all_collections,
            &"posts".to_string(),
            all_custom_structures,
        )
        .unwrap();

        let test_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        assert_eq!(
            test_collection,
            Err(String::from("Error: id is already in use"))
        );

        let test_collection =
            Collection::update_id(&mut all_collections, &"posts2".to_string(), "posts3");
        assert_eq!(
            test_collection,
            Err(String::from("Error: Collection not found"))
        );

        let test_collection =
            Collection::update_id(&mut all_collections, &"posts".to_string(), "posts;");
        assert_eq!(
            test_collection,
            Err(String::from("Error: new_id contains an invalid character"))
        );

        let test_collection =
            Collection::update_project_id(&mut all_collections, &"posts".to_string(), "konnect;");
        assert_eq!(
            test_collection,
            Err(String::from(
                "Error: project_id contains an invalid character"
            ))
        );

        let test_collection =
            Collection::update_name(&mut all_collections, &"posts".to_string(), "Pos>ts");
        assert_eq!(
            test_collection,
            Err(String::from("Error: name contains an invalid character"))
        );

        let test_collection = Collection::update_description(
            &mut all_collections,
            &"posts".to_string(),
            "To store blog posts@.",
        );
        assert_eq!(
            test_collection,
            Err(String::from(
                "Error: description contains an invalid character"
            ))
        );
    }
    save_all_collections(&all_collections, file_name.to_string(), &String::new());
}
