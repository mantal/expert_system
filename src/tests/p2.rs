use super::super::*;
use p2;
use token;
use token::Operators;
use rule;
use rule::Rule;

#[test]
fn get_file() {
    let file = p2::get_file("src/tests/in/basic");

    assert!(file == "test\n");
}

#[test]
#[should_panic]
fn get_file_error() {
    let file = p2::get_file("src/tests/in/404");
}

#[test]
fn facts() {
    let mut rules: Vec<Rule> = Vec::new();

    p2::facts("=AB".to_string(), &mut rules);

    assert!(rules.len() == 2);
    //assert!(rules[0].rule[0] == Operators::True);
}

#[test]
#[should_panic]
fn facts_syntax() {
    p2::facts("=++".to_string(), &mut Vec::new());
}

#[test]
fn facts_empty() {
    let mut rules: Vec<Rule> = Vec::new();
    
    p2::facts("=".to_string(), &mut rules);

    assert!(rules.len() == 0);
}
