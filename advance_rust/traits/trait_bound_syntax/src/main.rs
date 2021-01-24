pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String
}


impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool
}

impl Summary for Tweet {
	/*fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}*/
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content )
	}
}

pub trait Summary {
	fn summarize(&self) -> String;
}

// Trait Bound Syntax
// Used when you want to bound a trait to a single type
fn notify<T:Summary>(item: &T) -> String {
	format!("Breaking news! {}", item.summarize())
}


fn main() {
    let _tweet = Tweet {
    	username: String::from("horse_ebooks"),
    	content: String::from(
    		"of course , as you probably already know, people"
    	),
    	reply: false,
    	retweet: false,
    };

    println!("1 new tweet: {:?}", notify(&_tweet));

    let _article = NewsArticle {
    	headline: String::from("Penguins win the Stanley Cup Championship!"),
    	location: String::from("Pittsburgh, PA, USA"),
    	author: String::from("Iceburgh"),
    	content: String::from(
    		"The Pittsburgh Penguins once again are the best \
    		hockey team in the NHL."
    	)
    };

}
