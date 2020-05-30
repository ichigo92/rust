#[derive(Debug)]


struct Colour (i32, i32, i32);
struct Points (i32, i32, i32);

fn main() {
    let black = Colour(6, 9, 0);
    println!("{:?}", black);
    build(black);
    // Statement won't work, remember borrwing concept
    // println!("Check {:?}", black);
    
    let axis = Points(6, 9, 0);
    // expected struct `Colour`, found struct `Points`
    // build(axis);
}

fn build(x: Colour) {
	println!("{:#?}", x);
}
