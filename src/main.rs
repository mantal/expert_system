//use std::io;
//use std::env;

mod token {

	pub struct Token {
        pub variable: bool,
		pub priority: i32,
		pub exec: fn(&mut Vec<&Token>, usize) -> bool
	}
}

mod operators {

	use token::Token;

    pub static Negate: Token = Token { priority: 3000, exec: negate, variable: false };
	pub static And: Token = Token { priority: 2100, exec: and, variable: false };
	pub static Or: Token = Token { priority: 2000, exec: or, variable: false };
	pub static True: Token = Token { priority: 0, exec: _true, variable: true  };
	pub static False: Token = Token { priority: 0, exec: _false, variable: true };

    fn negate(expr: &mut Vec<&Token>, pos: usize) -> bool {
		let mut res = false;
		if !(expr[pos + 1].exec)(expr, pos + 1) {
			res = true;
		}
		expr.remove(pos + 1);
		expr.remove(pos);
		if res { expr.insert(pos, &True); }
		else { expr.insert(pos, &False); }
		res
	}

    fn and(expr: &mut Vec<&Token>, pos: usize) -> bool {
		let mut res = false;
		if (expr[pos - 1].exec)(expr, pos - 1) && (expr[pos + 1].exec)(expr, pos + 1) {
			res = true;
		}
		expr.remove(pos + 1);
		expr.remove(pos);
		expr.remove(pos - 1);
		if res { expr.insert(pos - 1, &True); }
		else { expr.insert(pos - 1, &False); }
		res
	}

    fn or(expr: &mut Vec<&Token>, pos: usize) -> bool {
		let mut res = false;
		if (expr[pos - 1].exec)(expr, pos - 1) || (expr[pos + 1].exec)(expr, pos + 1) {
			res = true;
		}
		expr.remove(pos + 1);
		expr.remove(pos);
		expr.remove(pos - 1);
		if res { expr.insert(pos - 1, &True); }
		else { expr.insert(pos - 1, &False); }
		res
	}

    fn _false(expr: &mut Vec<&Token>, pos: usize) -> bool {
		false
	}

	fn _true(expr: &mut Vec<&Token>, pos: usize) -> bool {
		true
	}
}

// QUESTION diffrfence entre fn foo(&mut i: i32) et fn foo(i: &mut i32)

use token::Token;

fn print_(e: &mut Vec<&Token>) {
    println!("Len: {}", e.len());
    for t in e {
        print!("var: {}", t.variable);
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

fn eval(expr: &mut Vec<&Token>) -> bool {
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

	//expr.push(&operators::False);
	//expr.push(&operators::Or);
	expr.push(&operators::True);
	expr.push(&operators::And);
	expr.push(&operators::Negate);
	expr.push(&operators::True);

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
