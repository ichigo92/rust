#[derive(Debug)]


struct Book {
	name: String,
	author: String,
	price: u16,
	availability: bool
}


fn main() {

	// immutable struct
    let _book_a = Book {
    	name: String::from("Book A"),
    	author: String::from("Author A"),
    	price: 500,
    	availability: true
    };

    // mutable struct
    let mut _book_b = Book {
    	name: String::from("Book B"),
    	author: String::from("Author B"),
    	price: 600,
    	availability: true
    };

    _book_b.name = String::from("Book BB");

    println!("{:#?}", _book_a);
    println!("{:#?}", _book_b);
}
