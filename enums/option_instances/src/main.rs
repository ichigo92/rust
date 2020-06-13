fn main() {
    let _num = Some(15);
    let _string = Some(String::from("Hi"));

    println!("{:?}", _num);
    println!("{:?}", _string);

    let _none: Option<i32> = None;
    println!("{:?}", _none);
}
