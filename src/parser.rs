use token::Token;
use token::Operators;

use std::fs::File;
use std::env;
use std::error::Error;

use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

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
	let mut expr : Vec<&Token> = Vec::new();
	let mut i = 0;

    while i < line.len() {
		match line.as_bytes()[i] {
			b'(' => {
                println!("c'est un '(' au {} charactere", i);
                expr.push(&Operators::Bracket_open)
            },
			b')' => {
                println!("c'est un ')' au {} charactere", i);
                expr.push(&Operators::Bracket_close)
            },
			b'!' => {
                println!("c'est un '!' au {} charactere", i);
                expr.push(&Operators::Negate)
            },
			b'+' => {
                println!("c'est un '+' au {} charactere", i);
                expr.push(&Operators::And)
            },
			b'|' => {
                println!("c'est un '|' au {} charactere", i);
                expr.push(&Operators::Or)
            },
			b'^' => {
                println!("c'est un '^' au {} charactere", i);
                expr.push(&Operators::Xor)
            },
            b'A' ... b'Z' => {
                println!("c'est une variable au {} charactere", i);
                //TODO expr = Operators::push_var_named(expr, line.as_bytes()[i] as char)
            },
            b'\r' | b'\t' | b'\n' | b' ' => {
                println!("space au {} charactere", i);
            },
			b'=' => {
                i += 1;
                if line.as_bytes()[i] == b'>' {
                    //dosomething();
                    println!("c'est un '=>' du {} au  {} charactere", i-1, i)
                }
                else {
                    panic!("caractere N.{} indefinit vouliez vous ecrire => ?", i)
                }
            },
			b'<' => {
                i += 1;
                if line.as_bytes()[i] == b'=' && line.as_bytes()[i+1] == b'>' {
                    i += 1;
                    //dosomething();
                    println!("c'est un '<=>' du {} au  {} charactere", i-2, i)
                }
                else {
                    panic!("caractere N.{} indefinit vouliez vous ecrire <=> ?", i)
                }
            },
            _ => panic!("caractere N.{} indefinit", i),
		}
		i += 1;
	}
	println!("{}", line);
	return expr;
}

pub fn file_to_expr<'a>() -> Vec<&'a Token> {
	let mut f = get_file();
	let mut file = BufReader::new(&f);
    let mut expr : Vec<&Token> = Vec::new();
	for line in file.lines() {
		let l = line.unwrap();
		expr = line_processing(l);
	}
    return expr;
}

/*
 * ah but there is, .concat() on a vec of vecs or a slice of vecs
 * .append(), which drains the other vector.
 * there's also .extend_from_slice, .extend(), and .append(). The
 * last two can be used to move elements (no elementwise clone needed)
 */
