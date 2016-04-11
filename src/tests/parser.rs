use super::super::*;
use parser;
use token;
use token::Token;
use token::Operators;
use rule;
use rule::Rule;

//TODO move out
#[test]
fn get_file() {
    let file = parser::get_file("src/tests/in/basic".to_string());

    assert!(file == "test\n");
}

#[test]
#[should_panic]
fn get_file_error() {
    let file = parser::get_file("src/tests/in/404".to_string());
}

#[test]
fn facts() {
    let mut rules: Vec<Rule> = Vec::new();

    parser::facts("=AB".to_string(), &mut rules);

    assert!(rules.len() == 2);
    //TODO assert!(rules[0].rule[0] == Operators::True);
}

#[test]
#[should_panic]
fn facts_syntax() {
    parser::facts("=++".to_string(), &mut Vec::new());
}

#[test]
fn facts_empty() {
    let mut rules: Vec<Rule> = Vec::new();
    
    parser::facts("=".to_string(), &mut rules);

    assert!(rules.len() == 0);
}

#[test]
fn query() {
    let mut rules: Vec<Rule> = Vec::new();
    
    //TODO 

    assert!(true);
}

#[test]
#[should_panic]
fn query_syntax() {
    let mut rules: Vec<Rule> = Vec::new();
    
    parser::query("?[][][][][][]".to_string(), &mut rules);
}

#[test]
fn query_empty() {
    let mut rules: Vec<Rule> = Vec::new();
    
    parser::query("?".to_string(), &mut rules);
}

#[test]
fn rule() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr_ref: Vec<Token> = Vec::new();
    
    expr_ref.push(token::Operators::new_variable("π".to_string()));
    expr_ref.push(token::Operators::And());
    expr_ref.push(token::Operators::new_variable("π".to_string()));
    parser::rule("π + π ⇒ A".to_string(), &mut rules);

    
}

#[test]
fn TESTERRECURSION() { assert!(false); }

#[test]
fn to_rule() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr: Vec<Token> = Vec::new();

    expr.push(token::Operators::new_variable("A".to_string()));
    expr.push(token::Operators::Implies());
    expr.push(token::Operators::new_variable("B".to_string()));
    parser::expr_to_rule(&mut rules, &expr);
  
    assert!(rules.len() == 1);//TODO better
    assert!(rules[0].rule.len() == 1);

    expr.clear();
    rules.clear();
    
    expr.push(token::Operators::new_variable("A".to_string()));
    expr.push(token::Operators::Implies());
    expr.push(token::Operators::new_variable("B".to_string()));
    expr.push(token::Operators::And());
    expr.push(token::Operators::new_variable("C".to_string()));
    parser::expr_to_rule(&mut rules, &expr);
   
    println!("{:?}", rules[0].rule);
    assert!(rules.len() == 2);//TODO better
    assert!(rules[0].rule.len() == 3);
}

#[test]
#[should_panic]
fn to_rule_no_no_right_side() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr: Vec<Token> = Vec::new();

    expr.push(token::Operators::new_variable("A".to_string()));
    parser::expr_to_rule(&mut rules, &expr);
}

#[test]
#[should_panic]
fn to_rule_no_operand() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut expr: Vec<Token> = Vec::new();

    expr.push(token::Operators::Implies());
    parser::expr_to_rule(&mut rules, &expr);
}

