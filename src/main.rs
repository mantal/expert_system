mod token;

use token::Token;
use token::Operators;

fn print_(e: &mut Vec<&Token>) {
    println!("Len: {}", e.len());
    for t in e {
        print!("Type: {:?}", t.operator_type);
        //if t.variable {
        //    print!(", val: {}", (t.exec)(e, 0));
        //}
        println!("");
    }
}

fn get_next(expr: &mut Vec<&Token>) -> usize {
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

pub fn eval(expr: &mut Vec<&Token>) -> bool {
    let mut i: usize;

    while expr.len() > 1 {
        //println!("======\n");
        i = get_next(expr);
        //println!("i: {}", i);
        //print_(expr);
        (expr[i].exec)(expr, i);
    }
    return (expr[0].exec)(expr, 0);
}

fn main() {
	//let args: Vec<_> = env::args().collect();

	//todo later lexer
	// A + B; A = true, B = false
	let mut expr: Vec<&Token> = Vec::new();

	expr.push(&Operators::False);
	expr.push(&Operators::And);
	expr.push(&Operators::Bracket_open);
	expr.push(&Operators::True);
	expr.push(&Operators::Or);
	expr.push(&Operators::True);
	expr.push(&Operators::Bracket_close);

	let res = eval(&mut expr);
    println!("Result: {}\n", res);
}

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
