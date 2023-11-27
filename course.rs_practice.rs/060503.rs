struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v = Point(0, 127, 255);
    check_color(v);
}

fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}
