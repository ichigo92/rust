#[derive(Debug)]


struct Book {
	// missing lifetime specifier, expected lifetime specifier
	name: &str,
	// missing lifetime specifier, expected lifetime specifier
	author: &str,
	price: u16,
	availability: bool
}

fn main() {
    let _book_a = Book {
    	name: "Book A",
    	author: "Author A",
    	price: 500,
    	availability: true
    };
}
