// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

#[derive(Debug)]
struct Student {
    // * Use a struct containing the student's name and locker assignment
    // * The locker assignment should use an Option<i32>
    name: String,
    locker: Option<i32>,
}

fn main() {
    let jimmy = Student { name: "Jimmy".into(), locker: Some(132) };
    let ammy = Student { name: "Ammy".into(), locker: None };

    println!("Student's name: {:?}, locker: {:?}", jimmy.name, jimmy.locker);
    println!("Student's name: {:?}, locker: {:?}", ammy.name, ammy.locker);

    match jimmy.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assigned"),
    }
    match ammy.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assigned"),
    }
}
