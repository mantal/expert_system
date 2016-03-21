use token::Token;
use token::Operators;
use token::Operators::Value;

#[test]
fn  negate() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::Negate);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::Negate);
    expr.push(Operators::False);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Negate);
    expr.push(Operators::Unknow);
    assert!(super::super::eval(&mut expr) == Value::Unknow);
}

#[test]
fn brackets() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::Bracket_open);
    expr.push(Operators::True);
    expr.push(Operators::Bracket_close);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::True);
    expr.push(Operators::And);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::False);
    expr.push(Operators::Or);
    expr.push(Operators::True);
    expr.push(Operators::Bracket_close);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::True);
    expr.push(Operators::And);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::False);
    expr.push(Operators::Or);
    expr.push(Operators::True);
    expr.push(Operators::Bracket_close);
    expr.push(Operators::Bracket_close);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    // (T | F) + (F | T) => True
    expr.push(Operators::Bracket_open);
    expr.push(Operators::True);
    expr.push(Operators::Or);
    expr.push(Operators::False);
    expr.push(Operators::Bracket_close);
    expr.push(Operators::And);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::False);
    expr.push(Operators::Or);
    expr.push(Operators::True);
    expr.push(Operators::Bracket_close);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    // ((((T) & (T)))) => True
    expr.push(Operators::Bracket_open);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::True);
    expr.push(Operators::Bracket_close);
    expr.push(Operators::And);
    expr.push(Operators::Bracket_open);
    expr.push(Operators::True);
    expr.push(Operators::Bracket_close);
    expr.push(Operators::Bracket_close);
    expr.push(Operators::Bracket_close);
    expr.push(Operators::Bracket_close);
    assert!(super::super::eval(&mut expr) == Value::True);
}

#[test]
fn and() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::True);
    expr.push(Operators::And);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::False);
    expr.push(Operators::And);
    expr.push(Operators::False);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::False);
    expr.push(Operators::And);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::Unknow);
    expr.push(Operators::And);
    expr.push(Operators::False);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::Unknow);
    expr.push(Operators::And);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::Unknow);
}

#[test]
fn or() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::True);
    expr.push(Operators::Or);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::False);
    expr.push(Operators::Or);
    expr.push(Operators::False);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::False);
    expr.push(Operators::Or);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Unknow);
    expr.push(Operators::Or);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Unknow);
    expr.push(Operators::Or);
    expr.push(Operators::False);
    assert!(super::super::eval(&mut expr) == Value::Unknow);
}

#[test]
fn xor() {
    let mut expr: Vec<Token> = Vec::new();

    expr.push(Operators::True);
    expr.push(Operators::Xor);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::False);
    expr.push(Operators::Xor);
    expr.push(Operators::False);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(Operators::False);
    expr.push(Operators::Xor);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(Operators::Unknow);
    expr.push(Operators::Xor);
    expr.push(Operators::False);
    assert!(super::super::eval(&mut expr) == Value::Unknow);

    expr.clear();

    expr.push(Operators::Unknow);
    expr.push(Operators::Xor);
    expr.push(Operators::True);
    assert!(super::super::eval(&mut expr) == Value::Unknow);
}
