// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

// ###############################################
// my added requirements:
// store contacts on bigquery
// use multiple modules for code organization
// error check valid email using regexp

mod p2lib;

use p2lib::contacts::Contacts;
use p2lib::fileio::{read_db_file, write_db_file};
use p2lib::helpers::*;
use p2lib::menus::*;
use std::io::{self, Write};

fn main() {
    let dat_source = vec![
        r"C:\Code\rust\activities\data\p2_data.csv".to_owned(), // check first
        r"C:\Code\rust\activities\src\bin\p2_data.csv".to_owned(), // default if above doesn't exist
    ];

    let mut contact_list = Contacts::new(); // main prog mem

    let mut recs = match read_db_file(&dat_source, ',') {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(r) => r,
    };
    if recs.is_empty() {
        eprintln!("data sources return no records: {:?}", &dat_source);
        std::process::exit(1);
    }

    let headers = recs.remove(0);
    println!("loading data with headers: {:?}", headers);
    populate_hash(&mut contact_list, &recs);

    loop {
        match menu() {
            Some(1) => add_contact(&mut contact_list),
            Some(2) => delete_contact(&mut contact_list),
            Some(3) => edit_contact(&mut contact_list),
            Some(4) => contact_list.print_elems(),
            Some(5) => search_contacts(&contact_list),
            Some(_) => continue,
            None => {
                println!("goodbye!");
                break;
            }
        }
    }

    match write_db_file(&contact_list, &dat_source[0], ',') {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn menu() -> Option<u8> {
    println!("\nContact Manager:\n###############");
    println!("1) Add");
    println!("2) Delete");
    println!("3) Edit");
    println!("4) List");
    println!("5) Search");
    println!("Enter to Quit");
    print!("Command (1-5): ");
    io::stdout().flush().expect("flush borked somehow");

    // get option and parse
    loop {
        let selection = get_input("").trim().to_owned();
        if selection.is_empty() {
            return None;
        }

        match selection.parse::<u8>() {
            Ok(i) => return Some(i),
            Err(_) => {
                eprintln!("{} is not in 1-5", selection);
                continue;
            }
        };
    }
}

/*
    where are we!??

    - implement
    - " save to DB
    - " import from DB
        > if DB fails then use local file
*/
