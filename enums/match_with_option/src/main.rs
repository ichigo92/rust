fn main() {
    let four = Some(4);
    let sum = plus_one(four);

    println!("{:?}", sum);

    let none = plus_one(None);
    println!("{:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		Some(i) => Some(i + 1),
		None => None,
	}
}
