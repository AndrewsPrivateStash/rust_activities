// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

// My additions
// add persistant storage by writting to file when done and reading from file when loaded

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

impl Bill {
    fn new(n: &str, a: f64) -> Bill {
        Bill {
            name: n.to_owned(),
            amount: a,
        }
    }

    fn edit(&mut self, n: &str, a: f64) {
        self.name = n.to_owned();
        self.amount = a;
    }

    fn print(&self) -> String {
        format!("Name: {}; Amount Owed: ${:.2}", self.name, self.amount)
    }
}

fn make_storage(path: &str) -> HashMap<String, Bill> {
    let mut ret: HashMap<String, Bill> = HashMap::new();
    read_data(path, &mut ret).expect("could not read freom file");
    ret
}

fn main_menu() {
    println!("\nMenu:");
    println!("1) Add");
    println!("2) Delete");
    println!("3) Edit");
    println!("4) List");
    println!("5) Quit");
    print!("please enter a number (1-5): ");
    match io::stdout().flush() {
        // print! sits in buffer so must flush
        Ok(_) => (),
        Err(e) => {
            println!("hmm, a flush error occured\n{}", e);
            std::process::exit(1);
        }
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap(); //if this produces Err, panic
    buffer.trim().to_owned()
}

fn main() {
    // Storage
    let dat_source = r"C:\Code\rust\activities\data\bills.dat".to_owned();
    let mut bills = make_storage(&dat_source);

    loop {
        // print menu
        main_menu();
        let choice: String = get_input();

        // parse choice into int
        let int_choice = match choice.trim().parse::<u8>() {
            Ok(i) => i,
            Err(e) => {
                println!("can't parse: {}\n{}", choice, e);
                continue;
            }
        };

        // map choice to function
        match int_choice {
            1 => add_bill(&mut bills),
            2 => del_bill(&mut bills),
            3 => edit_bill(&mut bills),
            4 => list_bills(&bills),
            _ => {
                // quit
                println!("goodbye");
                if let Err(e) = write_data(&dat_source, &bills) {
                    println!("{}", e);
                }
                break;
            }
        }
    }
}

fn get_vec_input(args: i32, msg: &str) -> Result<Vec<String>, String> {
    println!("{}", msg);
    let in_str: String = get_input();
    let v: Vec<String> = in_str.split(',').map(|x| x.trim().to_string()).collect();

    //check for quit condition
    if v[0].to_lowercase() == "back" && v.len() == 1 {
        return Ok(v);
    }

    if args == -1 || v.len() == args as usize {
        Ok(v)
    } else {
        Err(format!("error: expected {} values, got {}", args, v.len()))
    }
}

fn add_bill(stg: &mut HashMap<String, Bill>) {
    loop {
        let msg = "please enter: id, name, bill_amount (or `back` to menu)";
        let v = match get_vec_input(3, msg) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        if v[0].to_lowercase() == "back" && v.len() == 1 {
            return;
        }

        let amt: f64 = match v[2].parse::<f64>() {
            Ok(f) => f,
            Err(e) => {
                println!("{} cannot be parsed into a f64\n{}", v[2], e);
                continue;
            }
        };

        stg.insert(v[0].to_owned(), Bill::new(&v[1], amt));
    }
}

fn del_bill(stg: &mut HashMap<String, Bill>) {
    loop {
        let msg = "enter ids of bills to remove, seperated by commas (or `back` to menu)";
        let v = match get_vec_input(-1, msg) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        if v[0].to_lowercase() == "back" && v.len() == 1 {
            return;
        }

        for id in v {
            match stg.remove_entry(&id) {
                Some((k, v)) => println!("removed id: {}, having record: {:?}", k, v),
                None => println!("no record for id: {}", id),
            }
        }
    }
}

fn edit_bill(stg: &mut HashMap<String, Bill>) {
    loop {
        let msg =
            "enter id of edit record followed by the new name and new amount\n(or `back` to menu)";
        let v = match get_vec_input(3, msg) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        if v[0].to_lowercase() == "back" && v.len() == 1 {
            return;
        }

        let amt: f64 = match v[2].parse::<f64>() {
            Ok(f) => f,
            Err(e) => {
                println!("{} cannot be parsed into a f64\n{}", v[2], e);
                continue;
            }
        };

        match stg.get_mut(&v[0]) {
            Some(b) => b.edit(&v[1], amt),
            None => {
                println!("could not find id: {}", v[0]);
                continue;
            }
        }
    }
}

fn list_bills(stg: &HashMap<String, Bill>) {
    for (k, v) in stg.iter() {
        println!("id: {} ==> {}", k, v.print());
    }
}

fn write_data(path: &str, stg: &HashMap<String, Bill>) -> std::io::Result<()> {
    let path = PathBuf::from(path);
    let mut file = File::create(path)?;

    let mut buffer = String::new();
    for (k, v) in stg.iter() {
        buffer.push_str(&format!("{},{},{:.2}\n", k, v.name, v.amount));
    }
    file.write(&buffer.as_bytes())
        .expect("could not write buffer");
    Ok(())
}

fn read_data(path: &str, stg: &mut HashMap<String, Bill>) -> std::io::Result<()> {
    let path = PathBuf::from(path);
    let file = File::open(path)?;
    for ln in io::BufReader::new(file).lines() {
        match &ln {
            Err(e) => {
                eprintln!("error reading dat file this line: {:?}\n{}", ln, e);
                continue;
            }
            Ok(s) => {
                let v: Vec<&str> = s.split(',').map(|x| x.trim()).collect();
                let amt: f64 = v[2].parse::<f64>().expect("bad parse on file input");
                stg.insert(v[0].to_owned(), Bill::new(&v[1], amt));
            }
        }
    }

    Ok(())
}
