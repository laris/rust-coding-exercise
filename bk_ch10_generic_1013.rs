/*
pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
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
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}
*/

mod bk_ch10_generic_1013_lib;
use bk_ch10_generic_1013_lib::*;

fn main() {
	let tweet = Tweet {
		username: String::from("hourse_ebooks"),
		content: String::from("of couse, as you probably already know, people"),
		reply: false,
		retweet: false,
	};
	println!("1 new tweet: {}", tweet.summarize());

}