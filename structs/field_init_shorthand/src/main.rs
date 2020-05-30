#[derive(Debug)]


struct Book {
	name: String,
	author: String,
	price: u16,
	availability: bool
}


fn main() {
    let book = build(String::from("Book A"), String::from("Author A"));
    println!("{:#?}", book);
}

fn build(name: String, author: String) -> Book {
	
	// Using field init shorthand
	Book {
		name,					// name: name
		author,					// author: author
		price: 500,
		availability: true,
	}
}
