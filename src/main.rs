extern crate regex;
use regex::Regex;

fn main() {
	// create a regular expression
	let re = Regex::new(r"^test$").unwrap();
	// use it with the is_match method
	assert!(re.is_match("test"));
	// now that we have used the is_match method, it will recognize that if we type
	// re.is_match, it will suggest is_match after typing the _
	// but if we try say re.find, it has nothing
	// some symbols in regex seem to work like Capture and CaptureNames
	// but the methods for regex::Regex seem broken at the very least
}
