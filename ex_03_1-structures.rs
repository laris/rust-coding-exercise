#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age:  u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  top_left:     Point,
  bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
  let Rectangle {
    top_left:     Point { x: tl_x, y: tl_y },
    bottom_right: Point { x: br_x, y: br_y },
  } = rect;
  (br_x - tl_x) * (tl_y - br_y)
}

fn square(point_low_left: Point, edge_len: f32) -> Rectangle {
  let Point { x: pll_x, y: pll_y } = point_low_left;
  Rectangle {
    top_left:     Point { x: pll_x, y: pll_y + edge_len },
    bottom_right: Point { x: pll_x + edge_len, y: pll_y },
  }
}

fn main() {
  let name = "Peter";
  let age  = 27;
  let peter = Person { name, age };

  println!("{:?}", peter);

  let point: Point = Point { x: 10.3, y: 0.4 };
  println!("point coordinates: ({}, {})", point.x, point.y);

  let bottom_right = Point {x: 5.2, ..point };
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  let Point { x: top_edge, y: left_edge } = point;
  let _rectangle = Rectangle {
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: bottom_right,
  };

  let _unit = Unit;

  let pair = Pair(1, 0.1);
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  let Pair(integer, decimal) = pair;
  println!("pair contains {:?} and {:?}", integer, decimal);

  println!("Rectangle area = {}", rect_area(_rectangle));
  println!("Squre rectangle = {:?}", square(point, 1.0));
}