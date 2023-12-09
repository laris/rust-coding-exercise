fn main() {
    // assert, _eq, _ne, debug_
    let a = 1;
    let b = 2;
    //assert!(a == b, "{} ne {}", a, b);
    //assert_eq!(a, b, "value should be equal");
    assert_ne!(a, b, "value should not be equal");
    //debug_assert!(a == b, "{} ne {}", a, b);
    //debug_assert_eq!(a, b, "value should be equal");
    debug_assert_ne!(a, b, "value should not be equal");
    // dbg!(&);
    #[derive(Debug)]
    enum RoomType {
        Bedroom,
        Kitchen,
    }
    #[derive(Debug)]
    struct Room {
        dimentions: (usize, usize),
        kind: RoomType,
    }
    let kitchen = Room {
        dimentions: (20, 20),
        kind: RoomType::Kitchen,
    };
    dbg!(&kitchen);

    // format!()
    let h = "Hello";
    let w = "World";
    let greet: String = format!("{h}{w}");
    println!("{greet}");

    // include_str!();
    let msg = include_str!("msg.txt");
    println!("{msg}");
    // include_bytes!();
    let bytes:&[u8] = include_bytes!("msg.txt");
    println!("{msg}");

    // env!();
    let config_user = env!("USER");
    println!("{config_user}");

    // todo unimplemented unreachable
    //todo!("taking a vacation");
    //unimplemented!("nobody wants this");

    let number = 12;
    let max_5 = {
        if number > 5 {
            5
        } else {
            number
        }
    };
    match max_5 {
        n @ 1..=5 => println!("n = {n}"),
        _ => unreachable!("n >5. this must is a bug!"),
    }
}
