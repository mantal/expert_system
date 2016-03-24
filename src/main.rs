mod tests;
mod token;
mod rule;
mod parser;

use token::Token;
use token::Operators;
use rule::Rule;
use parser::file_to_expr;

fn _print(e: &mut Vec<Token>) {
	println!("Len: {}", e.len());
	for t in e {
		print!("Type: {:?}", t.operator_type);
		//if t.variable {
		//    print!(", val: {}", (t.exec)(e, 0));
		//}
		println!("");
	}
}

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
    let mut expr2 = file_to_expr();
    expr2.pop();
	let res = eval(&Vec::new(), &mut expr2);
	println!("Result: {:?}\n", res);

}

//TODO TODO TODO rewrite logical op functions so that they dont eval variable unless needed

/*
   fn lexer(expr: &str, HashMap<str, Token>) -> Vec<Box<Token>> {

//tokens.insert("|", Token { priority: 1100, exec: or });

for e in expr.split_whitespace() {

}
Vec::new()
}
 */

/*
   faire un eval d'expr simple
   gerer les priotirtes
   gerer la syntaxe
   gerer les ()
   gerer les varaibles
   Gerer les regles simples =>
   Complexe <=>
   expand les regles
   A => B ^ C
   A + !C => B
   A + !B => C
   Gere les queries
   Parse fichier
   ???
   Profit
 */

/*

   Cas d'erreur
   - ()??

 */
