#[derive(Debug)]


struct Rectangle {
	width: u32,
	height: u32
}


//There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.

impl Rectangle {

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}


fn main() {
    
    let rect = Rectangle {
    	width: 8,
    	height: 8
    };
    println!("{:#?}", rect);
}
