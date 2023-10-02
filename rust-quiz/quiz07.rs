// this will allow compile without Enum::*
#![allow(bindings_with_variant_name)]

#[repr(u8)]
enum Enum {
    First,
    Second,
}

impl Enum {
    fn p(self) {
        match self {
            //Enum::First => print!("1"),
            //Enum::Second => print!("2"),
            Self::First => print!("1"),
            Self::Second => print!("2"),
            //First => print!("1"),
            //Second => print!("2"),
        }
    }
}

fn main() {
    Enum::p(unsafe {
        std::mem::transmute(1u8)
    });
}
