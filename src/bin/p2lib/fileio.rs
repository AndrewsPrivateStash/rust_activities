use super::contacts::Contacts;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn read_db_file(path: &[String], del: char) -> Result<Vec<Vec<String>>, String> {
    // grab file, and return table of values
    // del is the delimeter used in the file to seperate fields

    // catch empty vector
    if path.is_empty() {
        return Err("no paths defined to load file data".to_string());
    }

    let file = match get_file_from_paths(path) {
        Some(f) => {
            println!("using data file: {:?}", &f);
            f
        }
        None => return Err("no provided paths can be found for data files".to_string()),
    };

    let mut out_vec: Vec<Vec<String>> = Vec::new();
    for ln in io::BufReader::new(file).lines() {
        match &ln {
            Err(e) => {
                eprintln!("{}", *e);
                continue;
            }
            Ok(s) => {
                let v: Vec<String> = s.split(del).map(|x| x.trim().to_string()).collect();
                out_vec.push(v);
            }
        }
    }

    Ok(out_vec)
}

fn get_file_from_paths(path: &[String]) -> Option<File> {
    for p in path {
        match File::open(p) {
            Ok(f) => return Some(f),
            Err(_) => continue,
        }
    }
    None
}

pub fn write_db_file(c: &Contacts, path: &str, del: char) -> Result<(), String> {
    if c.get_len() == 0 {
        return Err("empty contacts, nothign to write.".to_string());
    }

    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(_) => return Err(format!("could not open to write: {}", path)),
    };

    let buffer = c.write_file_string(del);
    match file.write(buffer.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}
