use super::contacts::{Contacts, Email};
use super::helpers::parse_id;
use std::io::{self, Write};

pub fn get_input(s: &str) -> String {
    print!("{} ", s);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("read_line borked somehow");
    buffer.trim().to_owned()
}

pub fn add_contact(c: &mut Contacts) {
    println!("id is a number or blank, email is a string or blank");
    println!("Enter to go back");
    loop {
        let in_str = get_input("enter: id, firstname, lastname, email");
        if in_str.is_empty() {
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

pub fn delete_contact(c: &mut Contacts) {
    println!("enter in contact ids to remove from your contacts");
    println!("Enter to go back");
    loop {
        let in_str = get_input("enter: id(s) seperated by commmas:");
        if in_str.is_empty() {
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

pub fn edit_contact(c: &mut Contacts) {
    println!("enter contact id to edit fields in turn (Enter to go back)");
    loop {
        let in_str = get_input("enter a contact id to edit:");
        if in_str.is_empty() {
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
        if in_str.is_empty() {
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
        if in_str.is_empty() {
            return None;
        }
        if v.len() != 1 {
            eprintln!("expected one email address, got: {}", &in_str);
            continue;
        }
        return Email::new(v[0].to_owned());
    }
}

pub fn search_contacts(c: &Contacts) {
    // match first, last name or email (lowercase)
    let src = get_input("enter search string:");
    let v = c.search(&src);

    if let Some(r) = &v {
        for (i, c) in r.iter() {
            println!("found id: {}\t{}", i, c.print());
        }
    } else {
        println!("could not find any records matching: {}", &src);
    }
}
