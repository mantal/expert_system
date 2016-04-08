mod tests;
mod token;
mod rule;
mod p2;

use std::env;

use token::Token;
use token::Operators;
use rule::Rule;

fn get_next(expr: &mut Vec<Token>) -> usize {
	let mut i = 0;
	let mut max = 0;

	while i < expr.len() {
		if expr[i].priority > expr[max].priority {
			max = i;
		}
		i += 1;
	}
	max
}

pub fn eval(rules: &Vec<Rule>, expr: &mut Vec<Token>) -> Operators::Value {
	let mut i: usize;

	while expr.len() > 1 {
		i = get_next(expr);
		(expr[i].exec)(rules, expr, i);
	}
	return (expr[0].exec)(rules, expr, 0);
}

fn main() {
    let mut rules: Vec<Rule> = Vec::new();
    
    if std::env::args().count() != 2 {
        panic!("Usage: expert_system file");
    }
    p2::parse_file(p2::get_file(env::args().nth(1).unwrap()), &mut rules);
}

//TODO TODO TODO rewrite logical op functions so that they dont eval variable unless needed

