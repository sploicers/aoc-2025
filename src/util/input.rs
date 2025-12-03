pub fn filename_from_args() -> Option<String> {
	std::env::args().skip(1).next()
}
pub fn input_reader() -> std::io::BufReader<Box<dyn std::io::Read>> {
	std::io::BufReader::new(match filename_from_args() {
		Some(name) => Box::new(
			std::fs::File::open(&name)
				.expect(&format!("Fatal - could not open input file: {}", name)),
		),
		None => Box::new(std::io::stdin()),
	})
}

pub fn read_input_lines() -> impl Iterator<Item = String> {
	std::io::BufRead::lines(input_reader()).flat_map(|line| line)
}

pub fn regex_or_panic(pattern: &str) -> regex::Regex {
	regex::Regex::new(pattern).expect("Fatal - regex compilation failed")
}
