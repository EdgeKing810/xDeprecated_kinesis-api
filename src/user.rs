use crate::io::{fetch_file, save_file};

#[derive(Debug, Clone)]
pub enum Role {
    ROOT,
    ADMIN,
    AUTHOR,
}

impl Default for Role {
    fn default() -> Self { Role::AUTHOR }
}

#[derive(Default, Debug, Clone)]
pub struct User {
    id: String,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    role: Role
}

impl User {
    pub fn create_no_check(id: &str, first_name: &str, last_name: &str, username: &str, password: &str, role: Role) -> User {
        User {
            id: String::from(id),
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            username: String::from(username),
            password: String::from(password),
            role
        }
    }
}

pub fn fetch_all_users(path: String) -> Vec<User> {
    let all_users_raw = fetch_file(path.clone());

    let individual_users = all_users_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_users: Vec<User> = Vec::<User>::new();

    for user in individual_users {
        let current_user = user.split(";").collect::<Vec<&str>>();

        let parsed_role_raw = current_user[5].parse::<u32>();
        if let Err(e) = parsed_role_raw {
            println!("Error when parsing role in {}: {}", path, e);
            break;
        }
        
        let mut parsed_role: u32 = 2;
        if let Ok(val) = parsed_role_raw {
            parsed_role = val;
        }

        let role = match parsed_role {
            0 => Role::ROOT,
            1 => Role::ADMIN,
            _ => Role::AUTHOR
        };

        let tmp_user = User::create_no_check(current_user[0], current_user[1], current_user[2], current_user[3], current_user[4], role);
        final_users.push(tmp_user);
    }

    final_users
}

pub fn save_all_users(users: &Vec<User>, path: &str) {
    let mut stringified_users = String::new();

    for user in users {
        let number_role: u32 = match user.role {
            Role::ROOT => 0,
            Role::ADMIN => 1,
            _ => 2
        };

        stringified_users = format!(
            "{}{}{};{};{};{};{};{}",
            stringified_users,
            if stringified_users.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            user.id,
            user.first_name,
            user.last_name,
            user.username,
            user.password,
            number_role
        );
    }

    save_file(String::from(path), stringified_users);
    println!("Users saved!");
}

// TODO
// - REGISTER USER
// - LOGGING IN
// - HASHING PASSWORDS
