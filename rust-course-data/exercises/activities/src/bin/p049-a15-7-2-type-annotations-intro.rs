#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    fn print_many(msg: &str, count: i32) {}

    enum Mouse {
        LeftClick,
        RightClick,
        MiddleClick,
    }

    let num: i32 = 15;
    let num = 15;
    let a: char = 'a';
    let a = 'a';
    let left_click: Mouse = Mouse::LeftClick;
    let right_click = Mouse::RightClick;

    let numbers: Vec<i32> = vec![1, 2, 3];
    let letters: Vec<char> = vec!['a', 'b'];
    let clicks: Vec<Mouse> = vec![
        Mouse::LeftClick,
        Mouse::LeftClick,
        Mouse::RightClick,
    ];

}
