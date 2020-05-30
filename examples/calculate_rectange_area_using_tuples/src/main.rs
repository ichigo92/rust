fn main() {
    let rectangle = (100, 50);

    println!(
    	"The area of the rectangle is {}",
    	area(rectangle)
    );
}

fn area(dimensions: (u32, u32)) -> u32{
	dimensions.0 * dimensions.1
}
