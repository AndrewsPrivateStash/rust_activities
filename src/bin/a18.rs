// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: usize,
}

impl Customer {
    fn can_make_restricted(&self) -> Result<(), String> {
        match self.age {
            a if a >= 21 => Ok(()),
            _ => Err(format!("too young, only {} years old", self.age)),
        }
    }
}

fn main() {
    let customers: Vec<Customer> = vec![
        Customer {
            name: "peter".to_owned(),
            age: 18,
        },
        Customer {
            name: "paul".to_owned(),
            age: 24,
        },
        Customer {
            name: "james".to_owned(),
            age: 56,
        },
    ];

    for c in &customers {
        match c.can_make_restricted() {
            Ok(_) => println!("{} is old enough to buy the thing, at {}", c.name, c.age),
            Err(e) => println!("{} is not old enough!\t{}", c.name, e),
        }
    }
}
