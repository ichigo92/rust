fn main() {
    let _result = dangle();
    println!("{}", _result);
}


fn dangle() -> String {		// &String
	let _s = String::from("Hello");		//_s comes into scope
	_s 		// ownership is moved out of the function
	// &_s 	// address is moving out of the function to the calling function
}		//_s is dropped, memory is cleared in case of returning &_s
