pub trait Summary {
	fn summarize(&self) -> String {
		String::from("(Read more..)")
	}
	fn summarize_author(&self) -> String;
	fn summarize_include_author(&self) -> String {
		format!("(Read more from {}...)", self.summarize_author())
	}
}
/*
pub trait Summary {
	fn summarize(&self) -> String;
}
*/

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	/*
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
	*/
	fn summarize_author(&self) -> String {
		format!("@{}", self.author)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}

	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
}

fn main() {
	let tweet = Tweet {
		username: String::from("hourse_ebooks"),
		content: String::from("of couse, as you probably already know, people"),
		reply: false,
		retweet: false,
	};
	println!("1 new tweet: {}", tweet.summarize());

	let article = NewsArticle {
		headline: String::from("Penguins win the Stanley Cup Championship!"),
		location: String::from("Pittsburgh, PA, USA"),
		author: String::from("Iceburgh"),
		content: String::from("The Pittsburgh Penguins once again are the best
		hockey team in the NHL."),
	};

	println!("New article available! {}", article.summarize());
	println!("1 new tweet: {}", article.summarize_include_author());
}