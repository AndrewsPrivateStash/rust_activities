use super::contacts::Contacts;
use super::contacts::Email;

pub fn populate_hash(c: &mut Contacts, d: &[Vec<String>]) {
    // RULES
    // must have first two fields (otherwise drop)
    // if third is blank, then None value for email
    // if columns out of order, ignore for now

    let mut i: u64 = 0;
    for rec in d {
        i += 1;

        // skip empty records
        if rec.is_empty() {
            eprintln!("empty record: line {}", i);
            continue;
        }

        // skip if id is missing
        match rec.get(0) {
            None => continue,
            Some(v) if v.is_empty() => {
                eprintln!("missing id: {:?}, on line: {}", rec, i);
                continue;
            }
            _ => (),
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
        match names.len() {
            0..=1 => {
                eprintln!("missing first or last name: {:?}, on line: {}", rec, i);
                continue;
            }
            _ => c.add(
                parse_id(&rec[0]),
                &names[0],
                &names[1],
                Email::new(rec[2].to_owned()),
            ),
        }
    }
}

pub fn parse_id(s: &str) -> Option<usize> {
    match s.parse::<usize>() {
        Ok(i) => Some(i),
        Err(_) => {
            eprintln!("{} cannot be parsed into a usize", s);
            None
        }
    }
}

fn parse_name(s: &str) -> Vec<String> {
    s.split(' ').map(|x| x.trim().to_string()).collect()
}
