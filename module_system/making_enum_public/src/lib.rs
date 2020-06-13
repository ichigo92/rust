#![allow(dead_code)]
#![allow(unused_variables)]
mod back_of_house {
	#[derive(Debug)]
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

fn eat_at_restaurant() {
	let meal1 = back_of_house::Appetizer::Soup;
	let meal2 = back_of_house::Appetizer::Salad;
}