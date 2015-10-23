use token::Token;
use token::Operators;
use token::Operators::Value;

#[test]
fn  negate() {//TODO should be one assert / function
    let mut expr: Vec<&Token> = Vec::new();

    expr.push(&Operators::Negate);
    expr.push(&Operators::True);
    assert!(super::super::eval(&mut expr) == Value::False);

    expr.clear();

    expr.push(&Operators::Negate);
    expr.push(&Operators::False);
    assert!(super::super::eval(&mut expr) == Value::True);

    expr.clear();

    expr.push(&Operators::Negate);
    expr.push(&Operators::Unknow);
    assert!(super::super::eval(&mut expr) == Value::Unknow);
}
