fn longest(x: &str, y: &str) -> &str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
    let _string_one = String::from("abcd");
    let _string_two = "xyz";

    let _result = longest(_string_one.as_str(), _string_two);
    println!("The longest string is {}", _result);
}