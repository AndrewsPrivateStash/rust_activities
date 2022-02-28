// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#[derive(Debug)]
enum Color {
    White,
    Natural,
}

#[derive(Debug)]
struct Dims {
    length: u32,
    width: u32,
    heigth: u32,
}

struct Box {
    dim: Dims,
    weight: f32,
    color: Color,
}

impl Box {
    fn new(d: (u32, u32, u32), w: f32, c: Color) -> Self {
        Self {
            dim: Dims {
                length: d.0,
                width: d.1,
                heigth: d.2,
            },
            weight: w,
            color: c,
        }
    }

    fn print(&self) {
        println!(
            "dims: {:?}\nweight: {}\ncolor: {:?}",
            self.dim, self.weight, self.color
        );
    }
}

fn main() {
    let my_box = Box::new((10, 40, 2), 23.45, Color::White);
    my_box.print();
}
