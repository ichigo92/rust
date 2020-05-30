fn difference() {
	// Stored on heap memory, String type
    // Mutable
    let mut _a = String::from("Hello world");
    // Can be added becuase it is stored on heap
    // which has unlimited size
    _a.push_str(" People");
    println!("{}", _a);

    // Stored on stack memory, &str type (string literal)
    // Immutable
    let _m = "Hello world";
    // Cannot be added because it is stored in stack
    // which has fixed size
    _m.push_str(" People");
    println!("{}", _m);

}


fn main() {
    // String::from requests OS to provide memory to store "Hello"
    // The double colon (::) is an operator that allows us to namespace
    // this particular from function under the String type rather than
    // using some sort of name like string_from
    let _s = String::from("Hello");      // memory allocation
    // variable _s in memory
}   // _s is dropped now