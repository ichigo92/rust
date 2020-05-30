use std::io;


fn main() {
    // ::new() is an associated function
    // new() creates a new empty String
    let mut _s = String::new();

    // stdin is a function from the io module
    io:: stdin().read_line(&mut _s)
    	.expect("Failed to read a line");

   	println!("{}", _s);

   	// trim() Removes white spaces
   	// parse() Converts the String to another type.
   	// Annotation :u32 tell the string should be converted in unsigned integer of 32 bit
   	// parse() also return a io::Result type
   	let mut _age: u32 = _s.trim().parse()
   		.expect("Failed to read a line");

   	_age = _age + 1;
   	println!("{}", _age);
}
