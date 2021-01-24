#[derive(Debug)]
struct Point <T, U> {
	x: T,
	y: U,
}


fn main() {
	let integer_float = Point{x: 8, y: 10.9};
	let integer = Point{x: 8, y: 6};
	let float = Point{x: 7.6, y: 5.2};
	println!("{:?}", integer_float);
	println!("{:?}", float);
	println!("{:?}", integer);
}
