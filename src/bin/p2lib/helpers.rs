use super::contacts::Contacts;
use super::contacts::Email;

pub fn populate_hash(c: &mut Contacts, d: &[Vec<String>]) {
    // RULES
    // must have first two fields (otherwise drop)
    //      but, if only missing id, but has name (Email) then save til end and generate id
    // if third is blank, then None value for email
    // if columns out of order, ignore for now

    let mut post_process: Vec<Vec<String>> = Vec::new(); // list of records to impute ids

    let mut i: u64 = 1; // assumes header
    for rec in d {
        i += 1;

        // skip empty records
        if rec.is_empty() || rec.join("") == "" {
            eprintln!("empty record: line {}", i);
            continue;
        }

        // skip if id and name is missing
        if rec[0].is_empty() & rec.get(1).unwrap_or(&"".to_string()).is_empty() {
            eprintln!("missing id and name: {:?}, on line: {}", rec, i);
            continue;
        }

        // handle missing ids only
        if rec[0].is_empty() {
            print!("adding {:?} record to post-process list", &rec);
            post_process.push(rec.clone())
        }

        // skip if name is missing
        match rec.get(1) {
            None => {
                eprintln!("empty name: {:?}, on line: {}", rec, i);
                continue;
            }
            Some(v) if v.is_empty() => {
                eprintln!("empty name: {:?}, on line: {}", rec, i);
                continue;
            }
            _ => (),
        }

        let names = parse_name(&rec[1]);
        // skip names not having both first and last (if more then first two are used)
        if names.len() < 2 {
            eprintln!("missing first or last name: {:?}, on line: {}", rec, i);
            continue;
        }

        // skip malformed ids otherwise write record
        if let Some(i) = parse_id(&rec[0]) {
            c.add(Some(i), &names[0], &names[1], Email::new(rec[2].to_owned()))
        } else {
            eprintln!("skipping bad id: {}, on line: {}", &rec[0], i);
            continue;
        }
    }

    // post process missing ids
    for rec in &post_process {
        let names = parse_name(&rec[1]);
        if names.len() < 2 {
            eprintln!(
                "missing first or last name: {:?}, skipping post-proc record",
                rec
            );
            continue;
        }
        c.add(None, &names[0], &names[1], Email::new(rec[2].to_owned()))
    }
}

pub fn parse_id(s: &str) -> Option<usize> {
    match s.parse::<usize>() {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

fn parse_name(s: &str) -> Vec<String> {
    s.split(' ').map(|x| x.trim().to_string()).collect()
}
