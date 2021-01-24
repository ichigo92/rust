use std::fmt::Display;


fn main() {
    let _string_one = String::from("abcd");
    let _string_two = "xyz";

    let _result = longest_with_an_announcement(
    	_string_one.as_str(),
    	_string_two,
    	"Today is someone's birthday"
    );
    println!("The longest string is {}", _result);
}

fn longest_with_an_announcement<'a, T>(
	x: &'a str,
	y: &'a str,
	ann: T
) -> &'a str
where
	T: Display
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}