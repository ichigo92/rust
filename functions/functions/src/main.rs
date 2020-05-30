// fn main() {
//     println!("BAKER SHARING RECIPE WITH FIRST PERSON");
//     recipe();
// }

// fn recipe() {
// 	println!("1. Add milk");
//     println!("2. Add butter");
//     println!("3. Add eggs");
//     println!("4. Add sugar");
//     println!("5. Stir it");
//     println!("6. Heat on gentle flame");
// }

fn main() {
	let (value_x, value_y) = square(2, 9.1);
	println!("{}, {}", value_x, value_y);
}


fn square(x: u32, y: f64) -> (u32, f64) { // parameters
	let result_x = x * x;	// statement
	let result_y = y * y;	// statement

	// println!("Square of no {} is {}", x, result_x);
	// println!("Square of no {} is {}", y, result_y);
 
	(result_x, result_y)
}
