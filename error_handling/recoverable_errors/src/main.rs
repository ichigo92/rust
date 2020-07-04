use std::fs::File;


fn main() {
    let f = File::open("hello.txt");	// result type will be Result enum with Ok variant and Err variant

    let f = match f {
    	Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
