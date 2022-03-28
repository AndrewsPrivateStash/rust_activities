use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub fn read_db_file(path: &str, del: char) -> Result<Vec<Vec<String>>, String> {
    // grab file, and return table of values
    // del is the delimeter used in the file to seperate fields

    let path = PathBuf::from(path);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
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

pub fn write_db_file(path: &str, del: char) -> Result<(), String> {
    unimplemented!();
}
