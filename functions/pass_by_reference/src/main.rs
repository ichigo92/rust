fn main() {
    // let _s = String::from("Hello");
    // let _result = length(&_s);
    // println!("The length of the word {} is {} ", _s, _result);
    
    let mut _s = String::from("Hello");
    // change(&_s);
    change(&mut _s);
}

fn length(x: &String) -> usize {
	x.len()
}

// change(x: &String)		// The data it refers to cannot be borrowed as mutable
fn change(x: &mut String) {
	x.push_str(" World!");
	println!("{:?}", x);
}