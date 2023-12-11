//# regex = "*"
//# cached = "*"
// ^[a-z]{4}.*f\d[[:uper::]]$
// abcd 123 f2E
use cached::proc_macro::cached;
use regex::Regex;

#[cached]
fn date_regex() -> Regex {
    // Matches ISO 8601 dates: 2021-02-19
    const re: &'static str = r"\d{4}-\d{2}-\d{2}";
    Regex::new(re).expect("compilation failure")
}

fn main() {
    let text_str = r#"
        today is 2021-02-17
        tomorrow is 2021-02-18
        yesterday was 2021-02-16
    "#;
    if date_regex().is_match(text_str) { println!("matched");}
    if let Some(date) = date_regex().find(text_str) { println!("find first matched: {date:?}");}
    for date in date_regex().find_iter(text_str) { println!("find each matched: {date:?}");}
}
