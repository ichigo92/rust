// To print a struct or a non primitive data type
// we need to tell println! macro we need to use Debug trait
// rather than Display trait, to do that we add the following line

#[derive(Debug)]


struct Rectangle {
	width: u32,
	height: u32
}

fn main() {
    let rectangle = Rectangle { width: 50, height: 100 };

    // By default println! uses Display trait
    // This will throw error as Display trait cannot print a struct
    // println!("The area of the rectangle is {}", rectangle);
    
    println!("The area of the rectangle is {:?}", rectangle);

    // # tells the compiler to pretty print
    println!("The area of the rectangle is {:#?}", rectangle); 
    

    

}
