/**
 *
 * Solution to this problem is available in
 * traits/fixing_the_largest_function_with_trait_bounds
 *
 */

use std::fmt::Display;
use std::collections::HashMap;

#[derive(Debug)]
struct Pair<T> {
	x: T,
	y: T
}

impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}

fn main() {

	let mut _h = HashMap::new();
	_h.insert(1,2);

	let mut _h_one = HashMap::new();
	_h_one.insert(3,4);
	

	let _p = Pair::new(_h, _h_one);
	// let _p = Pair::new(7, 8);
	println!("{:#?}", _p);
	// _h_one.cmp_display();
}