use token::Token;
use token::Operators::Value;

#[derive(Clone)]
pub struct Rule<'a> {
    pub variable: char,
    pub rule: Vec<&'a Token>
}

pub fn query(rules: Vec<Rule>, var: char) -> Value {
    let mut arr = rules.iter().filter(|&e| e.variable == var)
                            .map(|ref e| super::eval(&mut e.rule.clone()))
                            .collect::<Vec<_>>();

    match arr.iter().peekable().peek() { None => return Value::Unknow, _ => 0, };
    if arr.iter().all(|&e| e == arr[0]) == false {
        panic!("Inconsistent rules");
    }
    arr[0]
}
