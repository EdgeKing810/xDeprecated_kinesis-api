use std::{fs, fs::File, io::prelude::*, io::BufReader};

pub fn fetch_file(path: String) -> String {
    let file = File::open(&path);
    let mut content = String::new();

    ensure_file_exists(&path);

    match file {
        Ok(f) => {
            let mut buf_reader = BufReader::new(f);
            let read_file = buf_reader.read_to_string(&mut content);
            if let Err(e) = read_file {
                println!("Error occured while reading file at {}: {}", path, e);
            }
        }
        _ => {}
    }

    content
}

pub fn ensure_file_exists(path: &String) {
    let file = File::open(&path);

    match file {
        Err(_) => {
            let create_file = File::create(&path);
            if let Err(e) = create_file {
                println!("Error occured while creating file at {}: {}", &path, e);
            }
        }
        _ => {}
    }
}

pub fn save_file(path: String, data: String) {
    ensure_file_exists(&path);
    let file = File::create(&path);

    if let Ok(mut f) = file {
        let write_file = f.write_all(data.as_bytes());

        if let Err(e) = write_file {
            println!("Error occured while writing file at {}: {}", &path, e);
        }
    }
}

pub fn remove_file(path: String) {
    ensure_file_exists(&path);
    let remove_file_result = fs::remove_file(&path);
    if let Err(e) = remove_file_result {
        println!("Error while removing file: {} ({})", e, path);
    }
}
