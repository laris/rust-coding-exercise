// first demo 
enum Status {
    Error(i32),
    Info,
    Warn,
}
// 2nd demo
enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird {
    age: usize,
    species: Species,
}
// 3rd demo
#[derive(PartialEq, Eq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}
// 4th demo
struct Vehicle {
    km: usize,
    year: usize,
}
#[rustfmt::skip]
fn main() {
    let status = Status::Error(5);
    match status {
        Status::Error(n @ 3) => println!("error three"),
        Status::Error(n @ 5..=6) => println!("error 5 or 6: {n}"),
        Status::Error(n @ 4..=10) => println!("error 4 to 10: {n}"),
        Status::Error(n @ 18 | n @ 19) => println!("error 18 or 19: {n}"),
        //Status::Error(n @ (if n > 20 && n <= 30)) => println!("error 21 to 30: {n}"),
        Status::Error(n) => println!("error three: {n}"),
        Status::Info    => println!("info"),
        Status::Warn    => println!("Warn"),
    }
    // 2nd demo
    let hawk = Bird { age: 13, species: Species::Hawk, };
    match hawk {
        Bird { age: 4, .. } => println!("4 year old bird"),
        Bird { age: 4..=10 | 15..=20, .. } => println!("4-10 or 15-20 year old"),
        Bird { species: Species::Finch, .. } => println!("Finch!"),
        Bird { .. } => println!("other bird"),
    }
    // 3rd demo
    let stage = 5;
    let diff = Difficulty::Normal;
    match stage {
        s if (s == 5 && diff == Difficulty::Easy) => println!("easy mode stage"),
        s if diff == Difficulty::Normal => println!("normal difficulty stage"),
        s @ 10 | s @ 15 => println!("stage 10 or 15"),
        s => println!("stage {stage}"),
    }
    // 4th demo
    let car = Vehicle {
        km: 80_000,
        year: 2020,
    };

    match car {
        Vehicle { km, year } if km == 0 && year == 2020 => println!("new 2020 year car"),
        Vehicle { km, ..   } if km <= 50_000 => println!("under 50 km"),
        Vehicle { km, ..   } if km >= 80_000 => println!("at least 80 km"),
        Vehicle { year, ..   } if year == 2020 => println!("make in 2020"),
        Vehicle { .. } => println!("other mileage"),
    }
}
