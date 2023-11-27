#[derive(Debug)]
enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message;3] = [
        Message::Quit,
        Message::Move{x: 1, y:3},
        Message::ChangeColor(255,255,0),
    ];

    for msg in msgs { show_msg(msg)}
}

fn show_msg(msg: Message) {
    println!("{msg:?}");
}
