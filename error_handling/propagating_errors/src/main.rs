use std::fs::File;
// use std::fs;
// use std::io;
// use std::io::Read;
// the above mentioned two statement can be written in one line as following
use std::io::{self, Read};

use std::error::Error;

/*fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}*/

fn main() {
	// A Shortcut for Propagating Errors: the ? Operator
	/*fn read_username_from_file() -> Result<String, io::Error> {

	    fs::read_to_string("hello.txt")

	    /*let mut s = String::new();

	    File::open("hello.txt")?.read_to_string(&mut s)?;
	    
	    Ok(s)*/

	    /*let mut f = File::open("hello.txt")?;
	    let mut s = String::new();
	    f.read_to_string(&mut s)?;
	    Ok(s)*/
	}*/

	// add import to make this work
	// use std::error::Error
	fn read_from_file() -> Result<(), Box<dyn Error>> {
		// let f = File::open("hello.txt")?;

		// To resolve this error
		// One technique is to change the return type of your function to be Result<T, E>
		// if you have no restrictions preventing that. The other technique is to use a match
		// or one of the Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.
 
		let	f = File::open("hello.txt")?;
		Ok(())

	}
	println!("{:#?}", read_from_file());

	// println!("{:#?}", read_username_from_file());
}