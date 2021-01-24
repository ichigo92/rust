use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

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

impl Display for NewsArticle {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{} {}", self.headline, self.author)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool
}

impl Summary for Tweet {	
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

impl Display for Tweet {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{} {}", self.username, self.content)
	}
}

pub trait Summary {
	fn summarize(&self) -> String;
}

// Impl Trait Syntax
/*fn notify(item: &(impl Summary + Display), item1: &(impl Summary + Display)) -> String {
	format!("Breaking news! {} {}", item.summarize(), item1.summarize())
}*/

// Trait Bound Syntax
fn notify<T: Summary + Display>(item: &T, item1: &T) -> String {
	format!("Breaking news! {} {}", item.summarize(), item1.summarize())
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

    let _article = NewsArticle {
    	headline: String::from("Penguins win the Stanley Cup Championship!"),
    	location: String::from("Pittsburgh, PA, USA"),
    	author: String::from("Iceburgh"),
    	content: String::from(
    		"The Pittsburgh Penguins once again are the best \
    		hockey team in the NHL."
    	)
    };
    
    println!("1 new tweet: {:?}", notify(&_tweet, &_tweet));

}
