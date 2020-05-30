#[derive(Debug)]


struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block
// on which data type I am implementing my method on
impl Rectangle {
	// purpose of `&`
	// which ever object will lie in this parameter will be immutable
	// immutable borrow
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}