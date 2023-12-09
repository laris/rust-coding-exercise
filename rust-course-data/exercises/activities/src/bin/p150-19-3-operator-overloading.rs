use std::ops::Add;

#[derive(Debug)]
struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Speed(self.0 + rhs.0)
    }
}

impl Add<u32> for Speed {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Speed(self.0 + rhs)
    }
}

#[derive(Debug)]
struct Letter(char);
impl Add<Self> for Letter {
    type Output = String;
    fn add(self, rhs: Self) -> Self::Output {
        format!("{}{}", self.0, rhs.0)
    }
}
impl Add<char> for Letter {
    type Output = String;
    fn add(self, rhs: char) -> Self::Output {
        format!("{}{}", self.0, rhs)
    }
}
use std::ops::Index;
#[derive(Debug)]
struct Hvac {
    current_temp: i16,
    max_temp: i16,
    min_temp: i16,
}
#[derive(Debug)]
enum Temp {
    Current,
    Max,
    Min
}
impl Index<Temp> for Hvac {
    type Output = i16;
    fn index(&self, temp: Temp) -> &Self::Output {
        match temp {
            Temp::Current => &self.current_temp,
            Temp::Max => &self.max_temp,
            Temp::Min => &self.min_temp,
        }
    }
}
fn main() {
    let fast = Speed(5) + Speed(3);
    println!("fast: {fast:?}");
    let fast = Speed(5) + 55;
    println!("fast: {fast:?}");

    println!("{}", Letter('H') + Letter('i'));
    println!("{}", Letter('H') + 'i');

    let env = Hvac {
        current_temp: 30,
        max_temp: 60,
        min_temp: 0,
    };
    let current = env[Temp::Current];
    println!("{current:?}");
}
