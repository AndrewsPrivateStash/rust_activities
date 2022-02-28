// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

impl Student {
    fn print(&self) {
        match self.locker {
            Some(v) => println!("{} is assigned locker: {}", self.name, v),
            None => println!("{} is assigned no locker", self.name),
        }
    }
}

fn main() {
    // make a students

    let students: Vec<Student> = vec![
        Student {
            name: "some guy".to_string(),
            locker: Some(42),
        },
        Student {
            name: "other guy".to_string(),
            locker: None,
        },
    ];

    for s in &students {
        s.print();
    }
}
