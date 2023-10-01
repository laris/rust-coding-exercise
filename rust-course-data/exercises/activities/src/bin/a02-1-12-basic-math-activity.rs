// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn fn_add(input_a: i32, input_b: i32) -> i32 {
    input_a + input_b 
}

fn display_result(value: i32) {
   println!("{:?}", value); 
}
fn main() {
    display_result(fn_add(1, 2));
}
