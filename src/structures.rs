use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Type {
    TEXT,
    EMAIL,
    PASSWORD,
    RICHTEXT,
    NUMBER,
    ENUM,
    DATE,
    MEDIA,
    BOOLEAN,
    UID,
    JSON,
    CUSTOM(String),
}

impl Default for Type {
    fn default() -> Self {
        Type::TEXT
    }
}

#[derive(Default, Debug, Clone)]
pub struct Structure {
    pub id: String,
    name: String,
    stype: Type,
    default: String,
    min: usize,
    max: usize,
    encrypted: bool,
    unique: bool,
    regex_pattern: String,
    array: bool,
}

impl Structure {
    pub fn create(
        all_structures: &mut Vec<Structure>,
        id: &str,
        name: &str,
        stype_txt: &str,
        default: &str,
        min: usize,
        max: usize,
        encrypted: bool,
        unique: bool,
        regex_pattern: &str,
        array: bool,
    ) -> Result<(), String> {
        if Self::exist(all_structures, id) {
            let new_id = Uuid::new_v4();
            return Self::create(
                all_structures,
                &*new_id.to_string(),
                name,
                stype_txt,
                default,
                min,
                max,
                encrypted,
                unique,
                regex_pattern,
                array,
            );
        }

        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: String = String::new();

        let new_structure = Structure {
            id: tmp_id.clone(),
            name: "".to_string(),
            stype: Type::default(),
            default: "".to_string(),
            min: 0,
            max: 0,
            encrypted: false,
            unique: false,
            regex_pattern: "".to_string(),
            array: false,
        };
        all_structures.push(new_structure);

        let id_update = Self::update_id(all_structures, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("Error: {}", e);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let name_update = Self::update_name(all_structures, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let type_update = Self::update_type(all_structures, &new_id, stype_txt);
            if let Err(e) = type_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let default_update = Self::update_default(all_structures, &new_id, default);
            if let Err(e) = default_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let min_update = Self::update_min(all_structures, &new_id, min);
            if let Err(e) = min_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let max_update = Self::update_max(all_structures, &new_id, max);
            if let Err(e) = max_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let encrypted_update = Self::update_encrypted(all_structures, &new_id, encrypted);
            if let Err(e) = encrypted_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let unique_update = Self::update_unique(all_structures, &new_id, unique);
            if let Err(e) = unique_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let regex_update = Self::update_regex(all_structures, &new_id, regex_pattern);
            if let Err(e) = regex_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if !has_error {
            let array_update = Self::update_array(all_structures, &new_id, array);
            if let Err(e) = array_update {
                has_error = true;
                println!("Error: {}", e);
                latest_error = e;
            }
        }

        if has_error {
            let delete_project = Self::delete(all_structures, &new_id);
            if let Err(e) = delete_project {
                println!("Error: {}", e);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_structures: &Vec<Structure>, id: &str) -> bool {
        let mut found = false;
        for structure in all_structures.iter() {
            if structure.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_id(
        all_structures: &mut Vec<Structure>,
        id: &String,
        new_id: &str,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *new_id {
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
            return Err(String::from(
                "Error: new_id does not contain enough characters",
            ));
        } else if String::from(new_id.trim()).len() > 100 {
            return Err(String::from("Error: new_id contains too many characters"));
        }

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_name(
        all_structures: &mut Vec<Structure>,
        id: &String,
        name: &str,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        if !String::from(name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ' ')
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

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.name = name.trim().to_string();
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_type(
        all_structures: &mut Vec<Structure>,
        id: &String,
        stype_txt: &str,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        let stype = match stype_txt {
            "text" => Type::TEXT,
            "email" => Type::EMAIL,
            "password" => Type::PASSWORD,
            "richtext" => Type::RICHTEXT,
            "number" => Type::NUMBER,
            "enum" => Type::ENUM,
            "date" => Type::DATE,
            "media" => Type::MEDIA,
            "bool" => Type::BOOLEAN,
            "uid" => Type::UID,
            "json" => Type::JSON,
            _ => Type::CUSTOM(String::from(stype_txt)),
        };

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.stype = stype;
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_default(
        all_structures: &mut Vec<Structure>,
        id: &String,
        default: &str,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.default = String::from(default.trim());
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_min(
        all_structures: &mut Vec<Structure>,
        id: &String,
        min: usize,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.min = min;
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_max(
        all_structures: &mut Vec<Structure>,
        id: &String,
        max: usize,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.max = max;
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_encrypted(
        all_structures: &mut Vec<Structure>,
        id: &String,
        encrypted: bool,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.encrypted = encrypted;
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_unique(
        all_structures: &mut Vec<Structure>,
        id: &String,
        unique: bool,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.unique = unique;
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_regex(
        all_structures: &mut Vec<Structure>,
        id: &String,
        regex_pattern: &str,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.regex_pattern = regex_pattern.trim().to_string();
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn update_array(
        all_structures: &mut Vec<Structure>,
        id: &String,
        array: bool,
    ) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.array = array;
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        Ok(())
    }

    pub fn delete(all_structures: &mut Vec<Structure>, id: &String) -> Result<(), String> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == id.to_string() {
                found_structure = Some(structure.clone());
                break;
            }
        }

        if let None = found_structure {
            return Err(String::from("Error: Structure not found"));
        }

        let updated_structures: Vec<Structure> = all_structures
            .iter_mut()
            .filter(|structure| structure.id != *id)
            .map(|structure| Structure {
                id: structure.id.clone(),
                name: structure.name.clone(),
                stype: structure.stype.clone(),
                default: structure.default.clone(),
                min: structure.min.clone(),
                max: structure.max.clone(),
                encrypted: structure.encrypted.clone(),
                unique: structure.unique.clone(),
                regex_pattern: structure.regex_pattern.clone(),
                array: structure.array.clone(),
            })
            .collect::<Vec<Structure>>();

        *all_structures = updated_structures;

        Ok(())
    }

    pub fn stringify(all_structures: &Vec<Structure>) -> String {
        let mut stringified_structures = String::new();

        for structure in all_structures {
            let stype_txt = match structure.stype.clone() {
                Type::TEXT => "text".to_string(),
                Type::EMAIL => "email".to_string(),
                Type::PASSWORD => "password".to_string(),
                Type::RICHTEXT => "richtext".to_string(),
                Type::NUMBER => "number".to_string(),
                Type::ENUM => "enum".to_string(),
                Type::DATE => "date".to_string(),
                Type::MEDIA => "media".to_string(),
                Type::BOOLEAN => "bool".to_string(),
                Type::UID => "uid".to_string(),
                Type::JSON => "json".to_string(),
                Type::CUSTOM(txt) => txt.clone(),
            };

            stringified_structures = format!(
                "{}{}{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                stringified_structures,
                if stringified_structures.chars().count() > 1 {
                    "%"
                } else {
                    ""
                },
                structure.id,
                structure.name,
                stype_txt,
                structure.default,
                structure.min,
                structure.max,
                structure.encrypted,
                structure.unique,
                structure.regex_pattern,
                structure.array,
            );
        }

        stringified_structures
    }
}
