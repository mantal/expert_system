use token::Token;
use token::Operators;

use std::fs::File;
use std::env;
use std::error::Error;

pub fn get_file() -> File {
	let args: Vec<_> = env::args().collect();
	if args.len() > 1 {
		let mut file = match File::open(&args[1]) {
			Ok(f) => {
				f
			},
			Err(e) => {
				panic!("error message: {}", Error::description(&e))
			},
		};
		return file;
	}
	else { panic!("No file in argument"); }
}

pub fn line_processing<'a>(line: String) -> Vec<&'a Token> {
	let mut vol : Vec<&Token> = Vec::new();
	let mut i = 0;

while i < line.len() {
		match line.chars()[i] {
			'+' => println!("c'est un '+' au {} charactere", i),
			'(' => println!("c'est un '(' au {} charactere", i),
			')' => println!("c'est un ')' au {} charactere", i),
		}
		i += 1;
	}
	println!("{}", line);
	return vol;
}
