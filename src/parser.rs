use token::Token;
use token::Operators;
use rule::Rule;

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

pub fn line_processing(line: String) -> Vec<Token> {
	let mut expr : Vec<Token> = Vec::new();
	let mut i = 0;

    while i < line.len() {
		match line.as_bytes()[i] {
			b'(' => {
                println!("c'est un '(' au {} charactere", i);
                expr.push(Operators::Bracket_open)
            },
			b')' => {
                println!("c'est un ')' au {} charactere", i);
                expr.push(Operators::Bracket_close)
            },
			b'!' => {
                println!("c'est un '!' au {} charactere", i);
                expr.push(Operators::Negate)
            },
			b'+' => {
                println!("c'est un '+' au {} charactere", i);
                expr.push(Operators::And)
            },
			b'|' => {
                println!("c'est un '|' au {} charactere", i);
                expr.push(Operators::Or)
            },
			b'^' => {
                println!("c'est un '^' au {} charactere", i);
                expr.push(Operators::Xor)
            },
            b'A' ... b'Z' => {
                println!("c'est une variable au {} charactere", i);
                expr.push(Operators::new_variable(line.as_bytes()[i] as char))
            },
            b'\r' | b'\t' | b'\n' | b' ' => {
                println!("space au {} charactere", i);
            },
            b'#' => {
                return expr;
            },
			b'=' => { // implique =>
                i += 1;
                if line.as_bytes()[i] == b'>' {
                    println!("c'est un '=>' du {} au  {} charactere", i-1, i);
                    expr.push(Operators::implies)
                }
                else {
                    panic!("caractere N.{} indefinit vouliez vous ecrire => ?", i)
                }
            },
			b'<' => { // si et seulement si <=>
                i += 1;
                if line.as_bytes()[i] == b'=' && line.as_bytes()[i+1] == b'>' {
                    i += 1;
                    println!("c'est un '<=>' du {} au  {} charactere", i-2, i);
                    expr.push(Operators::if_and_only_if)
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

pub fn file_to_expr() -> Vec<Token> {
	let mut f = get_file();
	let mut file = BufReader::new(&f);
    let mut expr : Vec<Token> = Vec::new();
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

//TODO HANDLE SYNTAX ERROR
//TODO expr to str
pub fn to_rule(rules:  &mut Vec<Rule>, expr: &Vec<Token>) {
    let i = match expr.iter().position(|e| e.operator_type == Operators::Type::implies
                                       || e.operator_type == Operators::Type::if_and_only_if) {
        Some(e) => e,
        None => panic!("Syntax error: rule has no right side"),
    };
    let mut left = expr.clone();
    let mut right = left.split_off(i);
    right.remove(0);
    match right.len() {
        0 => panic!("Syntax error: right side has no operand"),
        1 => {
            match right[0].operator_type {
                Operators::Type::Operand{name} => (),
                _ => panic!("Syntax error: right side has no operand"),
            }
            rules.push(Rule { variable: right[0].get_name(), rule: left.clone() });
        },
        2 => {
            left.insert(0, Operators::Bracket_open);
            left.push(Operators::Bracket_close);
            left.insert(0, Operators::Negate);
            rules.push(Rule { variable: right[0].get_name(), rule: left.clone() });
        }
        3 => {
            match right[0].operator_type {
                Operators::Type::Operand{name} => (),
                _ => panic!("Syntax error"),
            }
            match right[1].priority  {//right[1] == Token::And TODO better
                2200 => (),
                _ => panic!("Unsupported operation or syntax error"),//TODO better
            }
            match right[2].operator_type {
                Operators::Type::Operand{name} => (),
                _ => panic!("Syntax error"),
            }
            rules.push(Rule { variable: right[0].get_name(), rule: left.clone() });
            rules.push(Rule { variable: right[2].get_name(), rule: left.clone() });
        }
        _ => panic!("Unsupported operation"),
    }
}
