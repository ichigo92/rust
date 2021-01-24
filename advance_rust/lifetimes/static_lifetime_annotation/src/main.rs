#![allow(unused)]
fn main() {
	// this reference can live for the entire duration of the program
	// The text of this string is stored directly in the program’s binary,
	// which is always available. Therefore, the lifetime of all string literals is 'static
	let s: &'static str = "I have a static lifetime.";
}
