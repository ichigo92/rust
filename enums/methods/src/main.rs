#[derive(Debug)]
enum Message {
	Quit,
    Move { x: i32, y: i32 },	// anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		println!("{:?}", self);
	}
}



fn main() {
    let _quit = Message::Quit;
    let _write = Message::Write(String::from("Hello how are you?"));
    let _move = Message::Move{ x: 10, y: -9 };
    let _color = Message::ChangeColor(10, 20, 30);

    _write.call();
    _color.call();
}
