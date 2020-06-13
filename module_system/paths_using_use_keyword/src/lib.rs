mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Absolute Path
use crate::front_of_house::hosting;
// unidiometic way
// use crate::front_of_house::hosting::add_to_waitlist;

// Relative Path
// self will be deprecated in future, would not recommend using it
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {

	// Absolute Path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative Path
	front_of_house::hosting::add_to_waitlist();

	// Using use keyword
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // unidiometic way
    // add_to_waitlist();
}
