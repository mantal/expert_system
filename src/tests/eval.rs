use rule;
use rule::Rule;
use token::Token;
use token::Operators;
use token::Operators::Value;

#[test]
fn  negate() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::Negate());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::Negate());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Negate());
    expr.push(Operators::Unknow());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);
}

#[test]
fn brackets() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::Bracket_open());
    expr.push(Operators::True());
    expr.push(Operators::Bracket_close());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::True());
    expr.push(Operators::And());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::False());
    expr.push(Operators::Or());
    expr.push(Operators::True());
    expr.push(Operators::Bracket_close());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::True());
    expr.push(Operators::And());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::False());
    expr.push(Operators::Or());
    expr.push(Operators::True());
    expr.push(Operators::Bracket_close());
    expr.push(Operators::Bracket_close());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    // (T | F) + (F | T) => True
    expr.push(Operators::Bracket_open());
    expr.push(Operators::True());
    expr.push(Operators::Or());
    expr.push(Operators::False());
    expr.push(Operators::Bracket_close());
    expr.push(Operators::And());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::False());
    expr.push(Operators::Or());
    expr.push(Operators::True());
    expr.push(Operators::Bracket_close());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    // ((((T) & (T)))) => True
    expr.push(Operators::Bracket_open());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::True());
    expr.push(Operators::Bracket_close());
    expr.push(Operators::And());
    expr.push(Operators::Bracket_open());
    expr.push(Operators::True());
    expr.push(Operators::Bracket_close());
    expr.push(Operators::Bracket_close());
    expr.push(Operators::Bracket_close());
    expr.push(Operators::Bracket_close());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);
}

#[test]
fn and() {
    let mut expr: Vec<Token> = Vec::new();

    // T & T => T
    expr.push(Operators::True());
    expr.push(Operators::And());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    // F & F => F
    expr.push(Operators::False());
    expr.push(Operators::And());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    //F & T => F
    expr.push(Operators::False());
    expr.push(Operators::And());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    //U & F => F
    expr.push(Operators::Unknow());
    expr.push(Operators::And());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    //U & T => U
    expr.push(Operators::Unknow());
    expr.push(Operators::And());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);
}

#[test]
fn nand() {
    let mut expr: Vec<Token> = Vec::new();

    // T & T => F
    expr.push(Operators::True());
    expr.push(Operators::Nand());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    // F & F => T
    expr.push(Operators::False());
    expr.push(Operators::Nand());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    //F & T => T
    expr.push(Operators::False());
    expr.push(Operators::Nand());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    //U & F => T
    expr.push(Operators::Unknow());
    expr.push(Operators::Nand());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    //U & T => U
    expr.push(Operators::Unknow());
    expr.push(Operators::Nand());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);
}

#[test]
fn nor() {
    let mut expr: Vec<Token> = Vec::new();

    //T | T => F
    expr.push(Operators::True());
    expr.push(Operators::Nor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    //F | F => T
    expr.push(Operators::False());
    expr.push(Operators::Nor());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    //F | T => F
    expr.push(Operators::False());
    expr.push(Operators::Nor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    //U | T => F
    expr.push(Operators::Unknow());
    expr.push(Operators::Nor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    //U | F => U
    expr.push(Operators::Unknow());
    expr.push(Operators::Nor());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);
}

#[test]
fn or() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::True());
    expr.push(Operators::Or());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::False());
    expr.push(Operators::Or());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::False());
    expr.push(Operators::Or());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Unknow());
    expr.push(Operators::Or());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Unknow());
    expr.push(Operators::Or());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);
}

#[test]
fn xor() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::True());
    expr.push(Operators::Xor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::False());
    expr.push(Operators::Xor());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::False());
    expr.push(Operators::Xor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Unknow());
    expr.push(Operators::Xor());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);

    expr.clear();

    expr.push(Operators::Unknow());
    expr.push(Operators::Xor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);
}

#[test]
fn xnor() {
    let mut expr: Vec<Token> = Vec::new();

    //T ^ T => T
    expr.push(Operators::True());
    expr.push(Operators::Xnor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    //F ^ F => T
    expr.push(Operators::False());
    expr.push(Operators::Xnor());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::True);

    expr.clear();

    //F ^ T => F
    expr.push(Operators::False());
    expr.push(Operators::Xnor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::False);

    expr.clear();

    //U ^ F => U
    expr.push(Operators::Unknow());
    expr.push(Operators::Xnor());
    expr.push(Operators::False());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);

    expr.clear();

    //U ^ T => U
    expr.push(Operators::Unknow());
    expr.push(Operators::Xnor());
    expr.push(Operators::True());
    assert!(super::super::eval(&Vec::new(), &mut expr) == Value::Unknow);
}

#[test]
fn variable() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::True());
    rules.push(Rule { variable: "A".to_string(), rule: expr.clone() });
    assert!(rule::query(rules.clone(), "A".to_string()) == Value::True);

    expr.clear();
    expr.push(Operators::new_variable("A".to_string()));
    rules.push(Rule { variable: "B".to_string(), rule: expr.clone() });
    assert!(rule::query(rules.clone(), "A".to_string()) == Value::True);

    rules.clear();
    assert!(rule::query(rules.clone(), "A".to_string()) == Value::False);
}
