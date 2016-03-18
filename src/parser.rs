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
