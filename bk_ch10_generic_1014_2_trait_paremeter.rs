mod bk_ch10_generic_1013_lib;
use bk_ch10_generic_1013_lib::*;

pub fn notify(item: impl Summary) {
	println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: T) {
	// pub fn notify2<T: Summary>(item1: T, item2: T)
	// pub fn notify3(item: impl Summary + Display)
	// pub fn notify4<T: Summary + Display>(item: T)
	// fn some_function<T: Dispaly + Clone, U: Clone + Debug>(t: T, u: U)
	// fn some_function<T, U>(t: T, u: U)
	// where T: Display + Clone, U: Clone + Debug
	println!("Breaking news! {}", item.summarize());
}

// return type that implement traits
fn returns_summarizable() -> impl Summary {
	Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("of course"),
		reply: false,
		retweet: false,
	}
}

fn main() {
	let tweet = Tweet {
		username: String::from("hourse_ebooks"),
		content: String::from("of couse, as you probably already know, people"),
		reply: false,
		retweet: false,
	};

	//notify(tweet);
	notify2(tweet);

	println!("{}", returns_summarizable().summarize());
}