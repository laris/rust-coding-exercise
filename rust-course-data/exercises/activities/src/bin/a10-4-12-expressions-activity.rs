// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
fn print_big_or_small(is_gt_100: bool) {
    match is_gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    // * Use a boolean variable set to the result of
    //   an if..else expression to store whether the value
    //   is > 100 or <= 100
    let value = 10;
    //let is_gt_100 = if value > 100 { true } else { false };
    let is_gt_100 =      value > 100 ;//{ true } else { false };

    print_big_or_small(is_gt_100);
}
