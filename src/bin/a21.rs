// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

impl User {
    fn new(u: i32, n: String) -> Self {
        Self {
            user_id: u,
            name: n,
        }
    }
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let names = vec![
        "Sam".to_owned(),
        "Andrew".to_owned(),
        "Matt".to_owned(),
        "Katie".to_owned(),
        "Batty".to_owned(),
    ];

    for n in &names {
        print_if_found(n);
    }
}

fn print_if_found(name: &str) {
    // Print out the User struct if found, or a "not found" message if not

    let str = find_user(&name)
        .map(|i| User::new(i, name.to_owned()))
        .map(|s| format!("{:?}", s));

    match &str {
        Some(s) => println!("{}", s),
        None => println!("{} not found", &name),
    }
}
