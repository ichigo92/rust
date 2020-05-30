fn main() {
    let _s = String::from("Hello");		// _s comes into scope
    take_ownership(_s);		// _s is moved into take_ownership function

    // will throw an error because
    // _s is no longer accessible inside this function
    // println!("{}", _s);
    

    let num = 5;		// num comes into scope
    makes_copy(num);	// num is copied to the function makes_copy

    println!("From the main function {}", num);
}


fn take_ownership(x: String) {	// x comes into scope now
	println!("{}", x);
}	// x is out of scope, and will be dropped


fn makes_copy(x: i32) {
	println!("{}", x);
}