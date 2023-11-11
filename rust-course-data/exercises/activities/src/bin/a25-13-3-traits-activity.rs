// Topiside_c: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the sidegth of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the sidegth of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square {
    side: i32,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.side_a + self.side_b + self.side_c
    }
}

fn print_perimeter(shap: impl Perimeter) {
    println!("{:?}", shap.calculate_perimeter());
}

fn main() {
    let square = Square { side: 1 };
    let triangle = Triangle { side_a: 1, side_b: 2, side_c: 3, };
    print_perimeter(square); 
    print_perimeter(triangle); 
}
