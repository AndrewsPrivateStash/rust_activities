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

use p2lib::contacts::{Contacts, Email};
use p2lib::fileio::read_db_file;
use p2lib::helpers::{populate_hash, parse_id};
use std::io::{self, Write};

fn main() {
    let dat_source = r"C:\Code\rust\activities\data\p2_data.csv".to_owned();
    let mut contact_list = Contacts::new();

    // let's test the file read
    let mut recs = match read_db_file(&dat_source, ',') {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(r) => r,
    };
    if recs.len() == 0 {
        eprintln!("data sourced return no records: {}", dat_source);
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
        if selection == "" {
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

fn get_input(s: &str) -> String {
    print!("{} ", s);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("read_line borked somehow");
    buffer.trim().to_owned()
}

fn add_contact(c: &mut Contacts) {
    println!("id is a number or blank, email is a string or blank");
    println!("Enter to go back");
    loop {
        let in_str = get_input("enter: id, firstname, lastname, email");
        if in_str == "" {
            break;
        }
        let v: Vec<String> = in_str.split(',').map(|x| x.trim().to_string()).collect();
        if v.len() != 4 {
            eprintln!("{} is not exactly four comma seperated values", in_str);
            continue;
        }

        let id_val = match v[0].as_str() {
            "" => None,
            s => parse_id(s),
        };

        c.add(id_val, &v[1], &v[2], Email::new(v[3].to_owned()));
    }
}

fn delete_contact(c: &mut Contacts) {
    println!("enter in contact ids to remove from your contacts");
    println!("Enter to go back");
    loop {
        let in_str = get_input("enter: id(s) seperated by commmas:");
        if in_str == "" {
            break;
        }
        let v: Vec<String> = in_str.split(',').map(|x| x.trim().to_string()).collect();

        for id in &v {
            match parse_id(id) {
                None => continue,
                Some(i) => c.drop(i),
            }
        }
    }
}

fn edit_contact(c: &mut Contacts) {
    println!("enter contact id to edit fields in turn (Enter to go back)");
    loop {
        let in_str = get_input("enter a contact id to edit:");
        if in_str == "" {
            break;
        }
        let v: Vec<String> = in_str.split(',').map(|x| x.trim().to_string()).collect();
        if v.len() != 1 {
            println!("one at a time please");
            continue;
        }

        let id = match parse_id(v[0].as_str()) {
            None => {
                eprintln!("{} is not a valid id", v[0]);
                continue;
            }
            Some(i) => i,
        };

        match get_first_and_last() {
            None => (),
            Some(n) => c.edit_rec_name(id, n),
        }
        c.edit_rec_email(id, get_email());
    }
}

fn get_first_and_last() -> Option<(String, String)> {
    loop {
        let in_str = get_input("enter new first and last name seperated by space:");
        let v: Vec<String> = in_str.split(' ').map(|x| x.trim().to_string()).collect();
        if in_str == "" {
            return None;
        }
        if v.len() != 2 {
            eprintln!("expected first and last name, but got: {}", &in_str);
            continue;
        }
        return Some((v[0].to_owned(), v[1].to_owned()));
    }
}

fn get_email() -> Option<Email> {
    loop {
        let in_str = get_input("enter new email, or enter for none:");
        let v: Vec<String> = in_str.split(' ').map(|x| x.trim().to_string()).collect();
        if in_str == "" {
            return None;
        }
        if v.len() != 1 {
            eprintln!("expected one email address, got: {}", &in_str);
            continue;
        }
        return Email::new(v[0].to_owned());
    }
}

fn search_contacts(c: &Contacts) {
    // match first, last name or email (lowercase)
    let src = get_input("enter search string:");
    let v = c.search(&src);
    if v.is_none() {
        println!("could not find any records matching: {}", &src);
    } else {
        for (i, r) in v.unwrap().iter() {
            println!("found id: {}\t{}", i, r.print());
        }
    }
}

/*
    where are we!??

    - implement
    - " save to local file
    - " save to DB
    - " import from DB
        > if DB fails then use local file
*/
