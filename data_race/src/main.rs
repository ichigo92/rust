fn main() {
    let mut _s = String::from("Hello");
    
    // cannot borrow '_s' as mutable more than once at a time
    // solution, separate the mutable pointers
    // by putting them in different scopes
    {
    	let _a = &mut _s;
    	_a.push_str(" World");
    	println!("{}", _a);
    }
    {
    	let _b = &mut _s;
    	println!("{}", _b)
    }
}
