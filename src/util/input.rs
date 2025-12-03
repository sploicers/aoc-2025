use regex::Regex;
use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
};

pub fn filename_from_args() -> Option<String> {
	std::env::args().skip(1).next()
}

pub fn input_reader() -> impl BufRead {
	let reader: Box<dyn Read> = if let Some(name) = filename_from_args() {
		Box::new(File::open(&name).expect(&format!("Fatal - could not open input file: {}", name)))
	} else {
		Box::new(std::io::stdin())
	};

	BufReader::new(reader)
}

pub fn read_input_lines() -> impl Iterator<Item = String> {
	input_reader().lines().flat_map(|line| line)
}

pub fn regex_or_panic(pattern: &str) -> Regex {
	Regex::new(pattern).expect("Fatal - regex compilation failed")
}
