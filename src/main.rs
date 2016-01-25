extern crate regex;
use regex::Regex;

fn main() {
	let re = Regex::new(r"^test$").unwrap();
	assert!(re.is_match("test"));
}
