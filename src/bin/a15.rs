// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage { price: f32, name: String },
    Vip { price: f32, name: String },
    Standard(f32),
}

fn main() {
    // make a vector of tickets of each type
    let tickets = vec![
        Ticket::Backstage {
            price: 150.00,
            name: "Andrew Pfaendler".to_owned(),
        },
        Ticket::Vip {
            price: 75.00,
            name: "Joe Blow".to_owned(),
        },
        Ticket::Standard(25.00),
    ];

    for t in tickets {
        match t {
            Ticket::Backstage { price: p, name: n } => {
                println!("Backstage {{ price: {:.2}, name: {} }}", p, n)
            }
            Ticket::Vip { price: p, name: n } => println!("Vip {{ price: {:.2}, name: {} }}", p, n),
            Ticket::Standard(p) => println!("Standard {{ price: {:.2} }}", p),
        }
    }
}
