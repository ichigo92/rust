fn main() {
    let x: i8 = 5;

    let y: Option<i8> = Some(10);

    // Error: No implementation for `i8 + Option<i8>`
    // let sum = x + y;

    println!("{:?}", sum);
}
