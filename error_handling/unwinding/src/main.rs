fn main() {

	panic!("crash and burn"); // unwinds

    // first();
}

fn first() {
	// second();
}

fn second() {
	// Rust panicked here
}

// unwind - will flush everythin, meaning it will free the memory
// second() - all the variables, enums and structs will be cleaned
// first() - all the variables, enums and structs will be cleaned
// main