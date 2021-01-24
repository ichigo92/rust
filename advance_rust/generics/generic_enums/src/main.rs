// For Visualization Purposes
enum OptionI32 {
	Some(i32),
	None
}

enum OptionF64 {
	Some(f64),
	None
}

fn main() {

	// This process is called Monomorphization
	// At compile time compiler will create enum Option
	// which will take i32 values only
    let integer = Option::Some(5);
    // then it will create enum Option
    // which will take f64 values only
    let float = Option::Some(5.0);

    println!("{:#?}{:#?}", integer, float);
}
