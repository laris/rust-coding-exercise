fn main() {

    let my_numbers = vec![1, 2, 3];
    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.pop();
    my_numbers.len();

    let two = my_numbers[1];
    println!("my_numbers: {:?}\nmy_numbers' length: {:?}\ntwo: {}", my_numbers, my_numbers.len(), two);

    for num in my_numbers {
        println!("{:?}", num);

    }
}
