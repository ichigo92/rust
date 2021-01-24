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
fn notify<T:Summary>(item: &T, item1: &T) -> String {
	format!("Breaking news! {} {}", item.summarize(), item1.summarize())
}

// Impl Trait Syntax
/*fn notify(item: &impl Summary, item1: &impl Summary) -> String {
	format!("Breaking news! {} {}", item.summarize(), item1.summarize())
}*/


fn main() {
    let _tweet = Tweet {
    	username: String::from("horse_ebooks"),
    	content: String::from(
    		"of course , as you probably already know, people"
    	),
    	reply: false,
    	retweet: false,
    };

    let _article = NewsArticle {
    	headline: String::from("Penguins win the Stanley Cup Championship!"),
    	location: String::from("Pittsburgh, PA, USA"),
    	author: String::from("Iceburgh"),
    	content: String::from(
    		"The Pittsburgh Penguins once again are the best \
    		hockey team in the NHL."
    	)
    };

    // This will give error if using Trait Bound syntax
    // Error: mismatched types, expected struct `Tweet`, found struct `NewsArticle`
    // println!("1 new tweet: {:?}", notify(&_tweet, &_article));
    
    // This will work for Trait Bound syntax, as both parameters are of the same type
    println!("1 new tweet: {:?}", notify(&_tweet, &_tweet));

}
