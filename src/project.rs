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

    pub fn create(
        all_projects: &mut Vec<Project>,
        id: &str,
        name: &str,
        description: &str,
        api_path: &str,
    ) -> Result<(), String> {
        if !String::from(id)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(String::from("Error: id contains an invalid character"));
        }

        if String::from(id.trim()).len() < 1 {
            return Err(String::from("Error: id does not contain enough characters"));
        } else if String::from(id.trim()).len() > 100 {
            return Err(String::from("Error: id contains too many characters"));
        }

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

        for project in all_projects.iter() {
            if project.id.to_lowercase() == id.to_lowercase().trim() {
                return Err(String::from("Error: id already taken"));
            }

            if project.api_path.to_lowercase() == api_path.to_lowercase().trim() {
                return Err(String::from("Error: api_path already taken"));
            }
        }

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

        let new_project = Project {
            id: id.trim().to_string(),
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            api_path: api_path.trim().to_string(),
        };
        all_projects.push(new_project);

        Ok(())
    }
}

pub fn fetch_all_projects(path: String) -> Vec<Project> {
    let all_projects_raw = fetch_file(path.clone());

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

pub fn save_all_projects(projects: &Vec<Project>, path: &str) {
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

    save_file(String::from(path), stringified_projects);
    println!("Projects saved!");
}
