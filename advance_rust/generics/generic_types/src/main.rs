fn main() {
    let v = vec! [1, 2,3, 4, 5];
    let w = vec! [4, 3, 1, 2, 6];
    let x = vec! ['a', 'z', 'x'];

    println!("{}", largest(&v));
    println!("{}", largest(&w));
    println!("{}", largest(&x));
}


fn largest <T> (x: &[T]) -> T {
	let mut largest = x[0];

    for &item in x.iter() {
    	if item > largest {
    		largest = item;
    	}
    }

    largest
}