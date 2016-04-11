use token::Token;
use token::Operators::Value;

#[derive(Clone, Debug)]
pub struct Rule {
    pub variable: String,
    pub rule: Vec<Token>
}

static mut STACK_GUARD: i32 = 0;

pub fn query(rules: Vec<Rule>, var: String) -> Value {
    unsafe {
        STACK_GUARD += 1;
        if STACK_GUARD >= 200 {
            panic!("Aborting: recuring too deeply");
        }
    }
    let arr = rules.iter().filter(|&e| e.variable == var)
                            .map(|ref e| super::eval(&rules, &mut e.rule.clone()))
                            .collect::<Vec<_>>();

    match arr.iter().peekable().peek() { None => return Value::False, _ => 0, };
    if arr.iter().all(|&e| e == arr[0]) == false {
        panic!("Inconsistent rules");
    }
    arr[0]
}
