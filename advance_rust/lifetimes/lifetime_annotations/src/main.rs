fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

/**
 *  if we changed the implementation of the longest function
 *  to always return the first parameter rather than the longest
 *  string slice, we wouldn’t need to specify a lifetime on the y parameter
 */
/*fn longest<'a>(x: &'a str, y: &str) -> &'a str {
	x
}*/

/**
 *  If the reference returned does not refer to one of the parameters,
 *  it must refer to a value created within this function,
 *  which would be a dangling reference because the value will go out of
 *  scope at the end of the function
 */
/*fn longest<'a>(x: &str, y: &str) -> &'a str {
	let _result = String::from("really long string");
	result.as_str()
}*/



fn main() {

	// an example that shows that the lifetime of the reference in
	// result must be the smaller lifetime of the two arguments
	let _string_one = String::from("long string is long");
	let _result;
	{
		let _string_two = String::from("xyz");
		_result = longest(_string_one.as_str(), _string_two.as_str());
	}
	println!("The longest string is {}", _result);		// will throw an error


	// lifetime annotations restrict the longest function by passing
	// in references that have different concrete lifetimes
	/*let _string_one = String::from("long string is long");

	{
		let _string_two = String::from("xyz");
		let _result = longest(_string_one.as_str(), _string_two.as_str());
		println!("The longest string is {}", _result);
	}*/

	// Simple lifetime example
    /*let _string_one = String::from("abcd");
    let _string_two = "xyz";

    let _result = longest(_string_one.as_str(), _string_two);
    println!("The longest string is {}", _result);*/
}