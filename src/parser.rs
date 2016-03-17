use std::fs::File;
use std::env;

fn get_file()
{
	let args: Vec<_> = env::args().collect();
	if args.len() > 1
	{
		let mut file = match File::open(args[1])
		{
			Ok(f) => {
				f
			},
			Err(e) => {
				println!("{}", e);
				return;
			}
		}
	}
}
