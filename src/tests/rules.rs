use token::Operators;
use token::Operators::Value;
use rule::Rule;

#[test]
fn query() {
    let mut rules: Vec<Rule> = Vec::new();
    let a = [Operators::True()].to_vec();

    let mut res = super::super::rule::query(rules.clone(), "A".to_string());
    assert!(res == Value::False);

    rules.push(Rule { variable: "A".to_string(), rule: a});
    res = super::super::rule::query(rules.clone(), "A".to_string());
    assert!(res == Value::True);
}

#[test]
#[should_panic(expected = "Inconsistent rules")]
fn query_inconsistent() {
    let mut rules: Vec<Rule> = Vec::new();
    let rule_true = [Operators::True()].to_vec();
    let rule_false = [Operators::False()].to_vec();

    rules.push(Rule { variable: "A".to_string(), rule: rule_true});
    rules.push(Rule { variable: "A".to_string(), rule: rule_false});
    super::super::rule::query(rules.clone(), "A".to_string());
}
