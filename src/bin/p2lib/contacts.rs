use regex::Regex;
use std::collections::HashMap;
use std::fmt;

// lets have an email type to allow for ease of data validation
#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(e: String) -> Option<Self> {
        let re = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
        match re.is_match(&e) {
            true => Some(Self(e)),
            false => None,
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Contact {
    first: String,
    last: String,
    email: Option<Email>,
}

impl Contact {
    pub fn new(f: String, l: String, e: Option<Email>) -> Self {
        Self {
            first: clean_name_string(&f),
            last: clean_name_string(&l),
            email: e,
        }
    }

    pub fn edit_name(&mut self, name: (String, String)) {
        self.first = name.0;
        self.last = name.1;
    }

    pub fn edit_email(&mut self, e: Option<Email>) {
        self.email = e;
    }

    pub fn name_match(&self, n: &str) -> bool {
        match n.to_lowercase() {
            x if x == self.first.to_lowercase() => true,
            x if x == self.last.to_lowercase() => true,
            _ => false,
        }
    }

    pub fn email_match(&self, e: Option<Email>) -> bool {
        if self.email.is_none() || e.is_none() {
            return false;
        }
        matches!(e.as_ref().unwrap().0.to_lowercase(), x if x == self.email.as_ref().unwrap().0.to_lowercase())
    }

    pub fn print(&self) -> String {
        match &self.email {
            None => format!("{} {} : NULL", self.first, self.last),
            Some(e) => format!("{} {} : {}", self.first, self.last, e.0),
        }
    }

    fn print_email(&self) -> String {
        match &self.email {
            Some(e) => format!("{}", e),
            None => "".to_string(),
        }
    }

    pub fn get_write_line(&self, del: char) -> String {
        format!("{} {}{}{}", self.first, self.last, del, self.print_email())
    }
}

pub struct Contacts {
    inner: HashMap<usize, Contact>,
}

impl Contacts {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: Option<usize>, f: &str, l: &str, e: Option<Email>) {
        match id {
            Some(i) => {
                // explicit id passed, make sure it doesn't exist
                if self.inner.contains_key(&i) {
                    eprintln!("the id: {} already exists, skipping..", i);
                    return;
                }
                self.inner
                    .insert(i, Contact::new(f.to_owned(), l.to_owned(), e));
            }

            None => {
                // no id given, generate next id
                let new_id = self.next_id();
                self.inner
                    .insert(new_id, Contact::new(f.to_owned(), l.to_owned(), e));
            }
        }
    }

    pub fn next_id(&self) -> usize {
        // find max id, and return next id
        let max_id = self
            .inner
            .keys()
            .fold(0, |m: usize, x| std::cmp::max(m, *x));
        max_id + 1
    }

    pub fn print_elems(&self) {
        if self.inner.is_empty() {
            println!("EMPTY!");
            return;
        }

        for (k, v) in self.inner.iter() {
            println!("ID: {}, {}", k, v.print());
        }
    }

    pub fn drop(&mut self, id: usize) {
        self.inner.remove_entry(&id);
    }

    pub fn edit_rec_name(&mut self, id: usize, name: (String, String)) {
        let r = self.inner.get_mut(&id);
        match r {
            None => eprintln!("the id: {} was not found in contacts", id),
            Some(v) => v.edit_name(name),
        }
    }

    pub fn edit_rec_email(&mut self, id: usize, e: Option<Email>) {
        let r = self.inner.get_mut(&id);
        match r {
            None => eprintln!("the id: {} was not found in contacts", id),
            Some(v) => v.edit_email(e),
        }
    }

    pub fn search(&self, src: &str) -> Option<Vec<(usize, Contact)>> {
        let mut recs: Vec<(usize, Contact)> = Vec::new();

        for (k, v) in self.inner.iter() {
            if v.name_match(src) || v.email_match(Email::new(src.to_string())) {
                recs.push((*k, v.clone()))
            }
        }

        match recs.len() {
            0 => None,
            _ => Some(recs),
        }
    }

    pub fn get_len(&self) -> usize {
        self.inner.len()
    }

    pub fn write_file_string(&self, d: char) -> String {
        let mut output = "id,name,email\n".to_string();
        for (k, c) in self.inner.iter() {
            output.push_str(&format!("{}{}{}\n", k, d, c.get_write_line(d)))
        }
        output
    }
}

fn clean_name_string(s: &str) -> String {
    let mut clean_str = String::new();
    for c in s.to_lowercase().chars() {
        if c.is_alphabetic() || c == '-' || c == '\'' {
            clean_str.push(c);
        }
    }

    let mut cs = clean_str.chars();
    match cs.next() {
        None => clean_str,
        Some(c) => c.to_uppercase().collect::<String>() + cs.as_str(),
    }
}

#[cfg(test)]
mod test {
    use crate::p2lib::contacts;

    #[test]
    fn test_clean_name_string() {
        let vals = vec![
            ("Andrew", "anDrew"), // ( correct, (params) )
            ("Ru-pert", "ru-pert"),
            ("", ""),
            ("Andrew", "an;drEw"),
            ("Andrew's", "andRew's"),
        ];

        for v in vals {
            assert_eq!(contacts::clean_name_string(v.1), v.0.to_string());
        }
    }
}
