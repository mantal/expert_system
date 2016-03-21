#[derive(Copy)]
pub struct Token {
    pub operator_type: Operators::Type,
    pub priority: i32,
    pub exec: fn(&mut Vec<Token>, usize) -> Operators::Value
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            operator_type: self.operator_type,
			priority: self.priority,
            exec: self.exec
        }
    }
}

#[allow(unused_variables, dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
pub mod Operators {

    use token::Token;

    #[derive(Clone, Copy, Eq, PartialEq)]
    #[derive(Debug)]
    pub enum Type {
        Operand { name: char },
        Unary,
        Binary,
        Bracket_open,
        Bracket_close
    }

    pub static Negate: Token = Token { priority: 3000, exec: negate, operator_type: Type::Unary };
    pub static And: Token = Token { priority: 2200, exec: and, operator_type: Type::Binary };
    pub static Xor: Token = Token { priority: 2000, exec: xor, operator_type: Type::Binary };
    pub static Or: Token = Token { priority: 2100, exec: or, operator_type: Type::Binary };
    pub static True: Token = Token { priority: 0, exec: _true, operator_type: Type::Operand{ name: 't' } };
    pub static False: Token = Token { priority: 0, exec: _false, operator_type: Type::Operand { name: 'f' } };
    pub static Unknow: Token = Token { priority: 0, exec: unknow, operator_type: Type::Operand { name: 'f' } };
    pub static Bracket_open: Token = Token { priority: 4000, exec: bracket, operator_type: Type::Bracket_open };
    pub static Bracket_close: Token = Token { priority: -1, exec: _false, operator_type: Type::Bracket_close };
    pub static Variable: Token = Token { priority: 0, exec: variable, operator_type: Type::Operand { name: 'A' } };

    #[derive(Copy, Clone, Eq, PartialEq)]
    #[derive(Debug)]
    pub enum Value {
        True,
        False,
        Unknow
    }

    pub fn new_variable(name: char) -> Token {
        let mut res = Variable;

        res.operator_type = Type::Operand { name: name };
        res
    }
    
    fn variable(expr: &mut Vec<Token>, pos: usize) -> Value {
        Value::True // TODO HERE
    }

    fn unknow(expr: &mut Vec<Token>, pos: usize) -> Value {
        Value::Unknow
    }

    fn bracket(expr: &mut Vec<Token>, pos: usize) -> Value {
        expr.remove(pos);
        let mut i = pos;
        while i < expr.len() {
            if expr[i].operator_type == Type::Bracket_open {
                bracket(expr, i);
            }
            if expr[i].operator_type == Type::Bracket_close {
                let res = super::super::eval(&mut expr[pos..i].to_vec());
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

    fn negate(expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos + 1].exec)(expr, pos + 1);

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

    fn and(expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(expr, pos - 1);
        let b = (expr[pos + 1].exec)(expr, pos + 1);

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

    fn or(expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(expr, pos - 1);
        let b = (expr[pos + 1].exec)(expr, pos + 1);

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

    fn xor(expr: &mut Vec<Token>, pos: usize) -> Value {
        let a = (expr[pos - 1].exec)(expr, pos - 1);
        let b = (expr[pos + 1].exec)(expr, pos + 1);

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

    fn _false(expr: &mut Vec<Token>, pos: usize) -> Value {
        Value::False
    }

    fn _true(expr: &mut Vec<Token>, pos: usize) -> Value {
        Value::True
    }

    fn value_to_token(value: Value) -> Token {
        match value {
            Value::True     => True,
            Value::False    => False,
            _               => Unknow,
        }
    }
}
