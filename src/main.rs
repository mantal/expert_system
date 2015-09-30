//use std::io;
//use std::env;

mod token {

	pub struct Token {
		pub priority: i32,
		pub exec: fn(&mut Vec<&Token>, usize) -> bool
	}
}

mod operators {

	use token::Token;

	pub static Or: Token = Token { priority: 2100, exec: or };
	pub static True: Token = Token { priority: -1, exec: _true };
	pub static False: Token = Token { priority: -1, exec: _false };

	fn or(expr: &mut Vec<&Token>, pos: usize) -> bool {
		let mut res = false;
		if (expr[pos - 1].exec)(expr, pos - 1) && (expr[pos + 1].exec)(expr, pos + 1) {
			res = true;
		}
		expr.remove(pos + 1);
		expr.remove(pos);
		expr.remove(pos - 1);
		if res { expr.insert(pos - 2, &True); }
		else { expr.insert(pos - 2, &False); }
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

/*
** priority:
**		0 -> variables
**	 1000 -> unary operators
**	 2000 -> binary operators
**	 2100 -> binary operators (And)
**	 2200 -> binary operators (Or, Xor)
*/

use token::Token;

// todo a la place de mettre directement true ou false, mettre un token variable qui va se remplacer totu seuk par true ou false quand on l'eval et/ou se register

fn eval(expr: &mut Vec<&Token>) {//todo this
	
	let mut i = 0;
	while i < expr.len() && expr.len() > 1 {
		(expr[i].exec)(expr, i);
		i += 1;
	}
	println!("{}", (expr[0].exec)(expr, 0));
}

fn main() {
	//let args: Vec<_> = env::args().collect();

	//todo later lexer
	// A + B; A = true, B = false
	let mut expr: Vec<&Token> = Vec::new();

	expr.push(&operators::False);
	expr.push(&operators::Or);
	expr.push(&operators::False);
	expr.push(&operators::Or);
	expr.push(&operators::True);

	eval(&mut expr);
}

/*
fn lexer(expr: &str, HashMap<str, Token>) -> Vec<Box<Token>> {

	//tokens.insert("|", Token { priority: 1100, exec: or });

	for e in expr.split_whitespace() {

	}
	Vec::new()
}
*/

/* brainfuck
fn interpret(cmd: char, memory: &mut Vec<i32>,  ptr: &mut usize) {
	match cmd {
		'+' => memory[*ptr] += 1,
		'-' => memory[*ptr] -= 1,
		'>' => {
			if *ptr + 1== memory.len() { memory.push(0); }
			*ptr += 1;
		},
		'<' => {
			*ptr = if *ptr != 0 {*ptr - 1} else {memory.len() - 1};
		},
		_ => {},
	};
}

fn main() {
    
	let mut memory = vec![0];
	let mut ptr: usize;
	let mut program = String::new();

	io::stdin().read_line(&mut program).ok();
	
	ptr = 0;
	for cmd in program.chars() {
		interpret(cmd, &mut memory, &mut ptr);
	}

	for e in memory {
		println!("{}", e);
	}

}*/

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