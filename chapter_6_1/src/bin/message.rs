#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    Constant,
}

impl Message {
    const CONSTANT_VALUE: i32 = 5;

    fn call(&self) {
        match self {
            Message::Constant => println!("{:?}", Message::CONSTANT_VALUE),
            _ => println!("{:?}", self),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let c = Message::Constant;
    c.call();
}

// rustのenumで定数を持つことはできない
// そのため、enumを設定後にそれを表す文字列を別途実装する必要がある
