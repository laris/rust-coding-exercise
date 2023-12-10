fn main() {
    let mut vec_list = vec![];
    let msg = "Hello\nworld!";
    vec_list.push(msg);
    let msg = "Hello\tworld!";
    vec_list.push(msg);
    let msg = "left\\right";
    vec_list.push(msg);
    let msg = "over \"there\"";
    vec_list.push(msg);
    let smiley = "\u{1f642}";
    vec_list.push(smiley);

    // raw string
    let msg = r"Hello
world!";
    vec_list.push(msg);
    let msg = r"Hello    world!";
    vec_list.push(msg);
    let msg = r"left\right";
    vec_list.push(msg);
    let msg = r#"over "there""#;
    vec_list.push(msg);
    let msg = r##"over #"#there#"#"##;
    vec_list.push(msg);
    let smiley = r"🙂";
    vec_list.push(smiley);

    for msg in vec_list {
        println!("{msg}");
    }


}
