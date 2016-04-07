use rule::Rule;
use token::Token;
use token::Operators;

#[test]
fn to_rule() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr: Vec<Token> = Vec::new();

    expr.push(super::super::token::Operators::new_variable('A'));
    expr.push(super::super::token::Operators::implies);
    expr.push(super::super::token::Operators::new_variable('B'));
    super::super::parser::to_rule(&mut rules, &expr);
  
    assert!(rules.len() == 1);//TODO better

    expr.clear();
    rules.clear();
    
    expr.push(super::super::token::Operators::new_variable('A'));
    expr.push(super::super::token::Operators::implies);
    expr.push(super::super::token::Operators::new_variable('B'));
    expr.push(super::super::token::Operators::And);
    expr.push(super::super::token::Operators::new_variable('C'));
    super::super::parser::to_rule(&mut rules, &expr);
    
    assert!(rules.len() == 2);//TODO better
}

#[test]
#[should_panic]
fn to_rule_no_no_right_side() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr: Vec<Token> = Vec::new();

    expr.push(super::super::token::Operators::new_variable('A'));
    super::super::parser::to_rule(&mut rules, &expr);
}

#[test]
#[should_panic]
fn to_rule_no_operand() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr: Vec<Token> = Vec::new();

    expr.push(super::super::token::Operators::implies);
    super::super::parser::to_rule(&mut rules, &expr);
}

