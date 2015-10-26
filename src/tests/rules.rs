use token::Token;
use token::Operators;
use token::Operators::Value;
use rule::Rule;

#[test]
fn query() {
    let mut rules: Vec<Rule> = Vec::new();
    let a = [&Operators::True].to_vec();

    rules.push(Rule { variable: 'A', rule: a});
    let res = super::super::rule::query(rules, 'A');
    assert!(res == Value::True);
}
