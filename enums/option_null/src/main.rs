#[derive(Debug)]
enum Option<T> {
	Some(T),
	None
}

fn main() {
    let _num = Option::Some(5);
    let _string = Option::Some(String::from("Hi"));
    let _null: Option<i32> = Option::None;

    println!("{:?}", _num);
    println!("{:?}", _string);
    println!("{:?}", _null);
}
