use token::Operators;
use token::Operators::Value;
use rule::Rule;

#[test]
fn query() {
    let mut rules: Vec<Rule> = Vec::new();
    let a = [Operators::True].to_vec();

    let mut res = super::super::rule::query(rules.clone(), 'A');
    assert!(res == Value::Unknow);

    rules.push(Rule { variable: 'A', rule: a});
    res = super::super::rule::query(rules.clone(), 'A');
    assert!(res == Value::True);
}

#[test]
#[should_panic(expected = "Inconsistent rules")]
fn query_inconsistent() {
    let mut rules: Vec<Rule> = Vec::new();
    let rule_true = [Operators::True].to_vec();
    let rule_false = [Operators::False].to_vec();

    rules.push(Rule { variable: 'A', rule: rule_true});
    rules.push(Rule { variable: 'A', rule: rule_false});
    let res = super::super::rule::query(rules.clone(), 'A');
}
