fn main() {
    let _a: u8 = 10;
    let _b = &_a;	// b is a pointer which points to a
    let _c = &_b;	// c is a pointer which points to b
    println!("a: {}, b: {}, c: {}", _a, _b, _c);
    println!("The address of a is {:p}", _b);
    println!("The address of b is {:p}", _c);
}
