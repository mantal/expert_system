use token::Token;
use token::Operators::Value;

pub struct Rule<'a> {
    pub variable: char,
    pub rule: Vec<&'a Token>
}

pub fn query(rules: Vec<Rule>, var: char) -> Value {
    let a = rules.iter().filter(|&e| e.variable == var)
                            .map(|ref e| super::eval(&mut e.rule.clone()));
    for it in a {
        return it;//TODO check incoerences
    }
    Value::Unknow
}
