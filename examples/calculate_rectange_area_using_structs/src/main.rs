#[derive(Debug)]


struct Rectangle {
	width: u32,
	height: u32
}

fn main() {
    let rectangle = Rectangle {
    	width: 50,
    	height: 100
    };

    println!(
    	"The area of the rectangle is {}",
    	area(&rectangle)
    );


    // I want to use rectangle after i get area
    // error: value borrowed after move
    // to resolve add `&` before rectangle when
    // passing it as an `arguement` to the function area
    // Also add `&` to the `parameter` in area function
    println!("{:#?}", rectangle);
}


// You can have your instance borrowed by using `&` before parameter
fn area(rectangle: &Rectangle) -> u32{
	rectangle.width * rectangle.height
}
