// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Shape {
    fn perimeter(&self) -> Option<f64>;
}

struct Rectangle {
    length: f64,
    width: f64,
}

impl Shape for Rectangle {
    fn perimeter(&self) -> Option<f64> {
        match (self.length, self.width) {
            (a, b) if a < 0.0 || b < 0.0 => None,
            (_, _) => Some(2.0 * self.length + 2.0 * self.width),
        }
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Shape for Triangle {
    fn perimeter(&self) -> Option<f64> {
        match (self.a, self.b, self.c) {
            (a, b, c) if a < 0.0 || b < 0.0 || c < 0.0 => None,
            (_, _, _) => Some(self.a + self.b + self.c),
        }
    }
}

fn main() {
    let rec = Rectangle {
        length: 123.3456,
        width: 100.2,
    };
    let tri = Triangle {
        a: 2.1,
        b: 4.3,
        c: 5.0,
    };

    println!(
        "{} rec perimeter, {} tri perimeter",
        rec.perimeter().unwrap_or_default(),
        tri.perimeter().unwrap_or_default()
    );

    println!(
        "Rectangle peri {}",
        Rectangle::perimeter(&Rectangle {
            length: 2.4,
            width: 5.6
        })
        .unwrap()
    );
    println!(
        "Triangle peri {}",
        Triangle::perimeter(&Triangle {
            a: 2.4,
            b: 5.6,
            c: 10.123
        })
        .unwrap()
    );
}
