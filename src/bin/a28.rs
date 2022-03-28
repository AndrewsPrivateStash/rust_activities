// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug, Clone)]
struct Shoe(Color);

#[derive(Debug, Clone)]
struct Shirt(Color);

#[derive(Debug, Clone)]
struct Pants(Color);

trait PutOn {
    fn put_on_item(&self);
}

impl Shoe {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

impl PutOn for Shoe {
    fn put_on_item(&self) {
        println!("{:?}", self);
    }
}

impl Shirt {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

impl PutOn for Shirt {
    fn put_on_item(&self) {
        println!("{:?}", self);
    }
}

impl Pants {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

impl PutOn for Pants {
    fn put_on_item(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug, Clone)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

fn main() {
    let s = Shirt::new(Color::Black);
    let sh = Shoe::new(Color::Custom("balsam green".to_owned()));
    let p = Pants::new(Color::Red);

    s.put_on_item();
    sh.put_on_item();
    p.put_on_item();
}
