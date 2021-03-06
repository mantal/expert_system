use std::fmt;
use rule::Rule;

pub struct Token {
    pub operator_type: Operators::Type,
    pub priority: i32,
    pub exec: fn(&Vec<Rule>, &mut Vec<Token>, usize) -> Operators::Value
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            operator_type: self.operator_type.clone(),
            priority: self.priority,
            exec: self.exec
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token {{ type: {:?}, priority: {:?} }}", self.operator_type, self.priority)
    }
}

#[allow(unused_variables, dead_code, non_snake_case)]
pub mod Operators {

    use token::Token;
    use rule;
    use rule::Rule;

    #[derive(Clone, Eq, PartialEq)]
    #[derive(Debug)]
    pub enum Type {
        Operand { name: String },
        Unary,
        Binary,
        Bracket_open,
        Bracket_close,
        implies,
        if_and_only_if
    }

    impl Token {
        pub fn get_name(&self) -> String {
            match self.operator_type {
                Type::Operand{ref name} => name.to_string(),
                _ => panic!("Called get_name on a non variable token"),
            }
        }
    }

    pub fn Negate() -> Token { Token { priority: 3000, exec: negate, operator_type: Type::Unary } }
    pub fn And() -> Token { Token { priority: 2200, exec: and, operator_type: Type::Binary } }
    pub fn Nand() -> Token { Token { priority: 2200, exec: nand, operator_type: Type::Binary } }
    pub fn Or() -> Token { Token { priority: 2100, exec: or, operator_type: Type::Binary } }
    pub fn Nor() -> Token { Token { priority: 2000, exec: nor, operator_type: Type::Binary } }
    pub fn Xor() -> Token { Token { priority: 2000, exec: xor, operator_type: Type::Binary } }
    pub fn Xnor() -> Token { Token { priority: 2000, exec: xnor, operator_type: Type::Binary } }
    pub fn Bracket_open() -> Token { Token { priority: 4000, exec: bracket, operator_type: Type::Bracket_open } }
    pub fn Bracket_close() -> Token { Token { priority: -1, exec: _false, operator_type: Type::Bracket_close } }

    pub fn True() -> Token {
        Token { priority: 0, exec: _true, operator_type: Type::Operand{ name: "⊤".to_string() } }
    }

    pub fn False() -> Token {
        Token { priority: 0, exec: _false, operator_type: Type::Operand{ name: "⊤".to_string() } }
    }

    pub fn Unknow() -> Token {
        Token { priority: 0, exec: unknow, operator_type: Type::Operand{ name: "½".to_string() } }
    }
    pub fn Implies() -> Token { Token { priority: -1, exec: _false, operator_type: Type::implies } }
    pub fn Equivalent() -> Token { Token { priority: -1, exec: _false, operator_type: Type::if_and_only_if } }

    #[derive(Copy, Clone, Eq, PartialEq)]
    #[derive(Debug)]
    pub enum Value {
        True,
        False,
        Unknow
    }

    pub fn new_variable(name: String) -> Token {
        return Token { priority: 0, exec: variable, operator_type: Type::Operand { name: name } };
    }

    fn variable(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        rule::query(rules.clone(), expr[pos].get_name())
    }

    fn unknow(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        Value::Unknow
    }

    fn bracket(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        expr.remove(pos);
        let mut i = pos;
        while i < expr.len() {
            if expr[i].operator_type == Type::Bracket_open {
                bracket(rules, expr, i);
            }
            if expr[i].operator_type == Type::Bracket_close {
                let res = super::super::eval(rules, &mut expr[pos..i].to_vec());
                while expr[pos].operator_type != Type::Bracket_close {
                    expr.remove(pos);
                }
                expr.remove(pos);
                expr.insert(pos, value_to_token(res));
                i = 0;
                continue;
            }
            i += 1;
        }
        Value::True
    }

    fn negate(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos + 1].exec)(rules, expr, pos + 1);

        let res = match a {
            Value::False => Value::True,
            Value::True  => Value::False,
            _            => Value::Unknow,
        };

        expr.remove(pos + 1);
        expr.remove(pos);
        expr.insert(pos, value_to_token(res));

        res
    }

    fn and(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(rules, expr, pos - 1);
        let b = (expr[pos + 1].exec)(rules, expr, pos + 1);

        let res = if a == Value::True && b == Value::True {
            Value::True
        } else if a == Value::False || b == Value::False {
            Value::False
        } else {
            Value::Unknow
        };

        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        expr.insert(pos - 1, value_to_token(res));

        res
    }

    fn nand(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(rules, expr, pos - 1);
        let b = (expr[pos + 1].exec)(rules, expr, pos + 1);

        let res = if a == Value::True && b == Value::True {
            Value::False
        } else if a == Value::False || b == Value::False {
            Value::True
        } else {
            Value::Unknow
        };

        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        expr.insert(pos - 1, value_to_token(res));

        res
    }

    fn or(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(rules, expr, pos - 1);
        let b = (expr[pos + 1].exec)(rules, expr, pos + 1);

        let res = if a == Value::True || b == Value::True {
            Value::True
        } else if a == Value::False && b == Value::False {
            Value::False
        } else {
            Value::Unknow
        };

        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        expr.insert(pos - 1, value_to_token(res));

        res
    }

    fn nor(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(rules, expr, pos - 1);
        let b = (expr[pos + 1].exec)(rules, expr, pos + 1);

        let res = if a == Value::True || b == Value::True {
            Value::False
        } else if a == Value::False && b == Value::False {
            Value::True
        } else {
            Value::Unknow
        };

        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        expr.insert(pos - 1, value_to_token(res));

        res
    }

    fn xor(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(rules, expr, pos - 1);
        let b = (expr[pos + 1].exec)(rules, expr, pos + 1);

        let res = if a == Value::Unknow || b == Value::Unknow {
            Value::Unknow
        } else if a == Value::True && b == Value::True || a == Value::False && b == Value::False {
            Value::False
        } else {
            Value::True
        };

        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        expr.insert(pos - 1, value_to_token(res));

        res
    }

    fn xnor(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(rules, expr, pos - 1);
        let b = (expr[pos + 1].exec)(rules, expr, pos + 1);

        let res = if a == Value::Unknow || b == Value::Unknow {
            Value::Unknow
        } else if a == Value::True && b == Value::True || a == Value::False && b == Value::False {
            Value::True
        } else {
            Value::False
        };

        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        expr.insert(pos - 1, value_to_token(res));

        res
    }

    fn _false(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        Value::False
    }

    fn _true(rules: &Vec<Rule>, expr: &mut Vec<Token>, pos: usize) -> Value {
        Value::True
    }

    //TODO impl in Value
    fn value_to_token(value: Value) -> Token {
        match value {
            Value::True     => True(),
            Value::False    => False(),
            _               => Unknow(),
        }
    }
}
