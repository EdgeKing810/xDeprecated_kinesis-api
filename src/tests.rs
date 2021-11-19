#[cfg(test)]
use crate::{
    io::remove_file,
    mappings::{fetch_all_mappings, save_all_mappings, Mapping},
    project::{fetch_all_projects, save_all_projects, Project},
    user::{fetch_all_users, save_all_users, User},
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

    save_all_mappings(&all_mappings, file_name);
}

#[test]
fn test_users() {
    let file_name: &str = "data/users_test.txt";
    remove_file(file_name.to_string());

    let mut all_users = fetch_all_users(file_name.to_string());
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

    save_all_users(&all_users, file_name);
}

#[test]
fn test_projects() {
    let file_name: &str = "data/projects_test.txt";
    remove_file(file_name.to_string());

    let mut all_projects = fetch_all_projects(file_name.to_string());
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
        Err(String::from("Error: id contains an invalid character"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test",
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
        "test",
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
        "test",
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
    assert_eq!(test_project2, Err(String::from("Error: id already taken")));

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: api_path already taken"))
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

    save_all_projects(&all_projects, file_name);
}
