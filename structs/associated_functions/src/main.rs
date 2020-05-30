#[derive(Debug)]


struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	
	// Associated functions are often used for constructors that will return a new instance of the struct
	// They are used as constructors
	fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    
    let result = Rectangle::square(8);
    println!("{:#?}", result);
}
