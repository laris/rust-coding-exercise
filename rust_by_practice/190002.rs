struct Meters(u32);

fn main() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);


    let n = Meters(i);
    assert_eq!(n.0.pow(2), 4);
}
