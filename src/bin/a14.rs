// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

#[derive(Debug)]
enum Color {
    Black,
    White,
    Blue,
}

struct Person {
    age: u8,
    first_name: String,
    last_name: String,
    full_name: String,
    fav_color: Color,
    pronoun: String,
}

impl Person {
    fn new(a: u8, f: &str, l: &str, c: Color, p: &str) -> Self {
        Self {
            age: a,
            first_name: f.to_owned(),
            last_name: f.to_owned(),
            full_name: format!("{} {}", f, l),
            fav_color: c,
            pronoun: p.to_owned(),
        }
    }

    fn print(&self) {
        println! {"{} is {} years old\n{} favorite color is: {:?}",
            self.full_name,
            self.age,
            self.pronoun,
            self.fav_color
        }
    }
}

fn main() {
    let people: Vec<Person> = vec![
        Person::new(42, "Andrew", "Pfaendler", Color::Blue, "His"),
        Person::new(39, "Joshua", "Pfaendler", Color::Black, "His"),
        Person::new(62, "Kathy", "Pfaendler", Color::White, "Her"),
    ];

    for p in &people {
        if p.age <= 100 {
            p.print();
        }
    }

    println!("you stored: {} people", &people.len());
}
