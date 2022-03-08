// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    match (first, second) {
        ("", "") => "".to_owned(),
        ("", _) => second.trim().to_owned(),
        (_, "") => first.trim().to_owned(),
        (_, _) => format!("{} {}", first.trim(), second.trim()),
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp() {
        let vals = vec![
            (-100, (-1000, -100, 0)), // ( correct, (params) )
            (100, (1000, 0, 100)),
            (0, (0, 0, 0)),
            (5, (5, 0, 10)),
            (5, (5, 0, 5)),
            (-5, (-5, -5, 0)),
        ];

        for v in vals {
            let p = v.1;
            assert_eq!(clamp(p.0, p.1, p.2), v.0);
        }
    }

    #[test]
    fn test_div() {
        let vals = vec![
            (None, (5, 0)), // ( correct, (params) )
            (Some(0), (0, 5)),
            (Some(1), (1, 1)),
            (Some(1), (3, 2)),
            (Some(-1), (-3, 2)),
        ];

        for v in vals {
            let p = v.1;
            assert_eq!(div(p.0, p.1), v.0);
        }
    }

    #[test]
    fn test_concat() {
        let vals = vec![
            ("a b", ("a", "b")), // ( correct, (params) )
            ("b", ("", "b")),
            ("a", ("a", "")),
            ("", ("", "")),
            ("hmm yep", ("hmm ", " yep")),
        ];

        for v in vals {
            let p = v.1;
            assert_eq!(concat(p.0, p.1), v.0);
        }
    }
}
