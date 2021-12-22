use crate::project::{fetch_all_projects, save_all_projects, Project};
use wasm_bindgen::prelude::*;

pub fn get_projects_from_str(projects: &str) -> Vec<Project> {
    let mut all_projects = Vec::<Project>::new();

    for project in projects.split("\n").collect::<Vec<&str>>() {
        let tmp_project = Project::from_string(project);
        all_projects.push(tmp_project);
    }

    all_projects
}

pub fn turn_projects_to_str(projects: Vec<Project>) -> String {
    let mut stringified_projects = String::new();

    for project in projects {
        stringified_projects = format!(
            "{}{}{}",
            stringified_projects,
            if stringified_projects.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Project::to_string(project.clone())
        );
    }

    stringified_projects
}

#[wasm_bindgen]
pub fn create_project(
    projects: &str,
    id: &str,
    name: &str,
    description: &str,
    api_path: &str,
) -> String {
    let mut all_projects = get_projects_from_str(projects);
    let result = Project::create(&mut all_projects, id, name, description, api_path);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_project_id(projects: &str, id: &str, new_id: &str) -> String {
    let mut all_projects = get_projects_from_str(projects);
    let result = Project::update_id(&mut all_projects, &id.to_string(), new_id);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_project_name(projects: &str, id: &str, name: &str) -> String {
    let mut all_projects = get_projects_from_str(projects);
    let result = Project::update_name(&mut all_projects, &id.to_string(), name);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_project_description(projects: &str, id: &str, description: &str) -> String {
    let mut all_projects = get_projects_from_str(projects);
    let result = Project::update_description(&mut all_projects, &id.to_string(), description);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn update_project_api_path(projects: &str, id: &str, api_path: &str) -> String {
    let mut all_projects = get_projects_from_str(projects);
    let result = Project::update_api_path(&mut all_projects, &id.to_string(), api_path);

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn delete_project(projects: &str, id: &str) -> String {
    let mut all_projects = get_projects_from_str(projects);
    let result = Project::delete(&mut all_projects, &id.to_string());

    if let Err(e) = result {
        return e;
    }

    String::from("OK")
}

#[wasm_bindgen]
pub fn fetch_projects(path: &str, encryption_key: &str) -> String {
    let all_projects = fetch_all_projects(String::from(path), &encryption_key.to_string());

    let mut stringified_projects = String::new();

    for project in all_projects {
        stringified_projects = format!(
            "{}{}{}",
            stringified_projects,
            if stringified_projects.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Project::to_string(project.clone())
        );
    }

    stringified_projects
}

#[wasm_bindgen]
pub fn save_projects(projects: &str, path: &str, encryption_key: &str) {
    let all_projects = get_projects_from_str(projects);

    save_all_projects(
        &all_projects,
        String::from(path),
        &encryption_key.to_string(),
    );
}
