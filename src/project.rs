use crate::io::{fetch_file, save_file};

#[derive(Default, Debug, Clone)]
pub struct Project {
    id: String,
    name: String,
    description: String,
    api_path: String,
}

impl Project {
    fn create_no_check(id: &str, name: &str, description: &str, api_path: &str) -> Project {
        Project {
            id: String::from(id),
            name: String::from(name),
            description: String::from(description),
            api_path: String::from(api_path),
        }
    }

    pub fn exist(all_projects: &Vec<Project>, id: &str) -> bool {
        let mut found = false;
        for project in all_projects.iter() {
            if project.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn create(
        all_projects: &mut Vec<Project>,
        id: &str,
        name: &str,
        description: &str,
        api_path: &str,
    ) -> Result<(), String> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: String = String::new();

        let new_project = Project {
            id: tmp_id.clone(),
            name: "".to_string(),
            description: "".to_string(),
            api_path: "".to_string(),
        };
        all_projects.push(new_project);

        let id_update = Self::update_id(all_projects, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("Error: {}", e);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let name_update = Self::update_name(all_projects, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let description_update = Self::update_description(all_projects, &new_id, description);
            if let Err(e) = description_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let api_path_update = Self::update_api_path(all_projects, &new_id, api_path);
            if let Err(e) = api_path_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if has_error {
            let delete_project = Self::delete(all_projects, &new_id);
            if let Err(e) = delete_project {
                println!("Error: {}", e);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn update_id(
        all_projects: &mut Vec<Project>,
        id: &String,
        new_id: &str,
    ) -> Result<(), String> {
        let mut found_project: Option<Project> = None;

        for project in all_projects.iter() {
            if project.id == new_id {
                return Err(String::from("Error: id is already in use"));
            }
        }

        if !String::from(new_id)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(String::from("Error: new_id contains an invalid character"));
        }

        if String::from(new_id.trim()).len() < 1 {
            return Err(String::from("Error: id does not contain enough characters"));
        } else if String::from(new_id.trim()).len() > 100 {
            return Err(String::from("Error: id contains too many characters"));
        }

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_project {
            return Err(String::from("Error: Project not found"));
        }

        Ok(())
    }

    pub fn update_name(
        all_projects: &mut Vec<Project>,
        id: &String,
        name: &str,
    ) -> Result<(), String> {
        let mut found_project: Option<Project> = None;

        if !String::from(name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
        {
            return Err(String::from("Error: name contains an invalid character"));
        }

        if String::from(name.trim()).len() < 1 {
            return Err(String::from(
                "Error: name does not contain enough characters",
            ));
        } else if String::from(name.trim()).len() > 100 {
            return Err(String::from("Error: name contains too many characters"));
        }

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.name = name.trim().to_string();
                break;
            }
        }

        if let None = found_project {
            return Err(String::from("Error: Project not found"));
        }

        Ok(())
    }

    pub fn update_description(
        all_projects: &mut Vec<Project>,
        id: &String,
        description: &str,
    ) -> Result<(), String> {
        let mut found_project: Option<Project> = None;

        if description.trim().len() > 0 && String::from(description).chars().any(|c| c == ';') {
            return Err(String::from(
                "Error: description contains an invalid character",
            ));
        }

        if String::from(description.trim()).len() > 400 {
            return Err(String::from(
                "Error: description contains too many characters",
            ));
        }

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.description = description.trim().to_string();
                break;
            }
        }

        if let None = found_project {
            return Err(String::from("Error: Project not found"));
        }

        Ok(())
    }

    pub fn update_api_path(
        all_projects: &mut Vec<Project>,
        id: &String,
        api_path: &str,
    ) -> Result<(), String> {
        let mut found_project: Option<Project> = None;

        for project in all_projects.iter() {
            if project.api_path == api_path {
                return Err(String::from("Error: api_path is already in use"));
            }
        }

        if !String::from(api_path)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '/')
        {
            return Err(String::from(
                "Error: api_path contains an invalid character",
            ));
        }

        if api_path.to_lowercase() != api_path {
            return Err(String::from(
                "Error: api_path should not contain uppercase alphabetical character(s)",
            ));
        }

        if String::from(api_path.trim()).len() < 1 {
            return Err(String::from(
                "Error: api_path does not contain enough characters",
            ));
        } else if String::from(api_path.trim()).len() > 50 {
            return Err(String::from("Error: api_path contains too many characters"));
        }

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.api_path = api_path.trim().to_string();
                break;
            }
        }

        if let None = found_project {
            return Err(String::from("Error: Project not found"));
        }

        Ok(())
    }

    pub fn delete(all_projects: &mut Vec<Project>, id: &String) -> Result<(), String> {
        let mut found_project: Option<Project> = None;

        for project in all_projects.iter_mut() {
            if project.id == id.to_string() {
                found_project = Some(project.clone());
                break;
            }
        }

        if let None = found_project {
            return Err(String::from("Error: Project not found"));
        }

        let updated_projects: Vec<Project> = all_projects
            .iter_mut()
            .filter(|project| project.id != *id)
            .map(|project| Project {
                id: project.id.clone(),
                name: project.name.clone(),
                description: project.description.clone(),
                api_path: project.api_path.clone(),
            })
            .collect::<Vec<Project>>();

        *all_projects = updated_projects;

        Ok(())
    }
}

pub fn fetch_all_projects(path: String, encryption_key: &String) -> Vec<Project> {
    let all_projects_raw = fetch_file(path.clone(), encryption_key);

    let individual_projects = all_projects_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_projects: Vec<Project> = Vec::<Project>::new();

    for project in individual_projects {
        let current_project = project.split(";").collect::<Vec<&str>>();

        let tmp_project = Project::create_no_check(
            current_project[0],
            current_project[1],
            current_project[2],
            current_project[3],
        );
        final_projects.push(tmp_project);
    }

    final_projects
}

pub fn save_all_projects(projects: &Vec<Project>, path: String, encryption_key: &String) {
    let mut stringified_projects = String::new();

    for project in projects {
        stringified_projects = format!(
            "{}{}{};{};{};{}",
            stringified_projects,
            if stringified_projects.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            project.id,
            project.name,
            project.description,
            project.api_path,
        );
    }

    save_file(path, stringified_projects, encryption_key);
    println!("Projects saved!");
}
