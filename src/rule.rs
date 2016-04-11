use token::Token;
use token::Operators::Value;

#[derive(Clone, Debug)]
pub struct Rule {
    pub variable: String,
    pub rule: Vec<Token>
}

pub fn query(rules: Vec<Rule>, var: String) -> Value {
    let arr = rules.iter().filter(|&e| e.variable == var)
                            .map(|ref e| super::eval(&rules, &mut e.rule.clone()))
                            .collect::<Vec<_>>();

    match arr.iter().peekable().peek() { None => return Value::False, _ => 0, };
    if arr.iter().all(|&e| e == arr[0]) == false {
        panic!("Inconsistent rules");
    }
    arr[0]
}
