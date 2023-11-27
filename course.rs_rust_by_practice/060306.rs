fn main() {
    let mut s = String::from("hello world");
    // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let ch = first_character(&s);
    println!("the first character is: {}", ch);
    s.clear();
}

fn first_character(s: &str) -> &str {
    &s[..1]
}
