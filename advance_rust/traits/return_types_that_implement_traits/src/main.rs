#[derive(Debug)]
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

#[derive(Debug)]
pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool
}

impl Summary for Tweet {	
	fn summarize(&self) -> String {
		format!("@{}: {}", self.username, self.content)
	}
}

pub trait Summary {
	fn summarize(&self) -> String;
}

// You are only allowed to return single data type if returning a Trait
// error: if and else have incompatible types
// expected struct `NewsArticle`, found struct `Tweet`
fn returns_summarizable(switch: bool) -> impl Summary {
	if switch {
		NewsArticle {
	    	headline: String::from("Penguins win the Stanley Cup Championship!"),
	    	location: String::from("Pittsburgh, PA, USA"),
	    	author: String::from("Iceburgh"),
	    	content: String::from(
	    		"The Pittsburgh Penguins once again are the best \
	    		hockey team in the NHL."
	    	)
	    }
	} else {
		Tweet {
	    	username: String::from("horse_ebooks"),
	    	content: String::from(
	    		"of course , as you probably already know, people"
	    	),
	    	reply: false,
	    	retweet: false,
	    }
	}

	
}


fn main() {
    
    // Error
    returns_summarizable(true);

}
