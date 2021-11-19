use crate::io::{fetch_file, save_file};
use pwhash::{bcrypt, unix};
use regex::Regex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Role {
    ROOT,
    ADMIN,
    AUTHOR,
}

impl Default for Role {
    fn default() -> Self {
        Role::AUTHOR
    }
}

#[derive(Default, Debug, Clone)]
pub struct User {
    id: String,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    password: String,
    role: Role,
}

impl User {
    fn create_no_check(
        id: &str,
        first_name: &str,
        last_name: &str,
        username: &str,
        email: &str,
        password: &str,
        role: Role,
    ) -> User {
        User {
            id: String::from(id),
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            username: String::from(username),
            email: String::from(email),
            password: String::from(password),
            role,
        }
    }

    pub fn register(
        all_users: &mut Vec<User>,
        first_name: &str,
        last_name: &str,
        username: &str,
        email: &str,
        password: &str,
        role_numeric: u32,
    ) -> Result<(), String> {
        return User::create(
            all_users,
            first_name,
            last_name,
            username,
            email,
            password,
            role_numeric,
        );
    }

    pub fn create(
        all_users: &mut Vec<User>,
        first_name: &str,
        last_name: &str,
        username: &str,
        email: &str,
        password: &str,
        role_numeric: u32,
    ) -> Result<(), String> {
        let id = Uuid::new_v4();
        let role = match role_numeric {
            0 => Role::ROOT,
            1 => Role::ADMIN,
            _ => Role::AUTHOR,
        };

        if !String::from(first_name)
            .chars()
            .all(|c| c.is_alphabetic() || c == ' ' || c == '-')
        {
            return Err(String::from(
                "Error: first_name contains an invalid character",
            ));
        }

        if String::from(first_name.trim()).len() < 1 {
            return Err(String::from(
                "Error: first_name does not contain enough characters",
            ));
        } else if String::from(first_name.trim()).len() > 100 {
            return Err(String::from(
                "Error: first_name contains too many characters",
            ));
        }

        if !String::from(last_name)
            .chars()
            .all(|c| c.is_alphabetic() || c == ' ' || c == '-')
        {
            return Err(String::from(
                "Error: last_name contains an invalid character",
            ));
        }

        if String::from(last_name.trim()).len() < 1 {
            return Err(String::from(
                "Error: last_name does not contain enough characters",
            ));
        } else if String::from(last_name.trim()).len() > 100 {
            return Err(String::from(
                "Error: last_name contains too many characters",
            ));
        }

        for user in all_users.iter() {
            if user.username.to_lowercase() == username.to_lowercase().trim() {
                return Err(String::from("Error: username already taken"));
            }

            if user.email.to_lowercase() == email.to_lowercase().trim() {
                return Err(String::from("Error: email already taken"));
            }
        }

        if !String::from(username)
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(String::from(
                "Error: username contains an invalid character",
            ));
        }

        if String::from(username.trim()).len() < 1 {
            return Err(String::from(
                "Error: username does not contain enough characters",
            ));
        } else if String::from(username.trim()).len() > 50 {
            return Err(String::from("Error: username contains too many characters"));
        }

        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if !email_regex.is_match(email) {
            return Err(String::from("Error: Invalid email address"));
        }

        if String::from(email.trim()).len() < 1 {
            return Err(String::from(
                "Error: email does not contain enough characters",
            ));
        } else if String::from(email.trim()).len() > 100 {
            return Err(String::from("Error: email contains too many characters"));
        }

        if String::from(password.trim()).len() < 7 {
            return Err(String::from(
                "Error: password should be longer than 7 characters",
            ));
        } else if String::from(password.trim()).len() > 100 {
            return Err(String::from("Error: password contains too many characters"));
        }

        if !String::from(password)
            .trim()
            .chars()
            .any(|c| c.is_alphabetic() && c.is_uppercase())
        {
            return Err(String::from(
                "Error: password should contain at least 1 uppercase alphabetic character",
            ));
        } else if !String::from(password)
            .trim()
            .chars()
            .any(|c| c.is_alphabetic() && c.is_lowercase())
        {
            return Err(String::from(
                "Error: password should contain at least 1 lowercase alphabetic character",
            ));
        } else if !String::from(password)
            .trim()
            .chars()
            .any(|c| c.is_numeric())
        {
            return Err(String::from(
                "Error: password should contain at least 1 number",
            ));
        } else if password.contains(';') {
            return Err(String::from(
                "Error: password contains a forbidden character (;)",
            ));
        }

        let new_user = User {
            id: id.to_string(),
            first_name: first_name.trim().to_string(),
            last_name: last_name.trim().to_string(),
            username: username.trim().to_string(),
            email: email.trim().to_string(),
            password: bcrypt::hash(password.trim()).unwrap().to_string(),
            role,
        };
        all_users.push(new_user);

        Ok(())
    }

    pub fn login(all_users: &mut Vec<User>, auth: &str, password: &str) -> Result<User, String> {
        let mut found_user: Option<User> = None;

        for user in all_users.iter() {
            if user.email == auth.to_string() || user.username == auth.to_string() {
                found_user = Some(user.clone());
                break;
            }
        }

        if let None = found_user {
            return Err(String::from("Error: User not found"));
        }

        let correct_password =
            unix::verify(password.trim(), &*found_user.as_ref().unwrap().password);
        if !correct_password {
            return Err(String::from("Error: Password mismatch"));
        }

        Ok(found_user.unwrap())
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

        let parsed_role_raw = current_user[6].parse::<u32>();
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
            _ => Role::AUTHOR,
        };

        let tmp_user = User::create_no_check(
            current_user[0],
            current_user[1],
            current_user[2],
            current_user[3],
            current_user[4],
            current_user[5],
            role,
        );
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
            _ => 2,
        };

        stringified_users = format!(
            "{}{}{};{};{};{};{};{};{}",
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
            user.email,
            user.password,
            number_role
        );
    }

    save_file(String::from(path), stringified_users);
    println!("Users saved!");
}
