fn main() {
    let height = 100;
    let width = 50;

    println!(
    	"The area of the rectangle is {}",
    	area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32{
	width * height
}
