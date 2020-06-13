#![allow(dead_code)]


mod front_of_house {
	// Add `pub` keyword so `hosting` can be accessed
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}

		fn take_payment() {}
	}
}

mod dining {
	pub fn eat_at_restaurant() {
		// Absolute Path
		// crate::customer_experience::front_of_house::hosting::add_to_waitlist();
		crate::front_of_house::hosting::add_to_waitlist();

		// Relative Path
		// customer_experience::front_of_house::hosting::add_to_waitlist();
		// front_of_house::hosting::add_to_waitlist();
		// failed to resolve: use of undeclared type or module `front_of_house`
		// i.e `front_of_house doesn't exist in dining module`
		// resolution is to use `super` keyword
		// front_of_house::hosting::add_to_waitlist();
		super::front_of_house::hosting::add_to_waitlist();
	}
}

fn serve_order() {}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		// cannot find function `serve_order` in this scope
		// serve_order();
		super::serve_order()
	}

	fn cook_order() {}
}
