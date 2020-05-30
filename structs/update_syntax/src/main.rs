#[derive(Debug)]


struct Book {
	name: String,
	author: String,
	price: u16,
	availability: bool
}

fn main() {
    let _book_a = Book {
    	name: String::from("Book A"),
    	author: String::from("Author A"),
    	price: 500,
    	availability: true
    };


    let _book_b = Book {
    	name: String::from("Book B"),
    	author: String::from("Author B"),
    	price: _book_a.price,
    	availability: _book_a.availability	
    };

    // Using struct update syntax
    let _book_c = Book {
    	name: String::from("Book C"),
    	author: String::from("Author C"),
    	.._book_a
    };

    println!("{:#?}", _book_b);
    println!("{:#?}", _book_c);
}
