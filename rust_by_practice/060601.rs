fn main() {
    assert_eq!(Number::One  as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
}

enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

enum Number2 {
//    Zero = 0.0,
//    One  = 1.0,
//    Two  = 2.0,
    Zero = 0,
    One  = 1,
    Two  = 2,
}
