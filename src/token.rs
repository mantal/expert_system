pub struct Token {
    pub operator_type: Operators::Type,
    pub priority: i32,
    pub exec: fn(&mut Vec<&Token>, usize) -> Value
}

pub mod Operators {

    use token::Token;

    #[derive(PartialEq)]
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

    pub enum Value {
        True,
        False,
        Unknow
    }

    fn variable(expr: &mut Vec<&Token>, pos: usize) -> Value {
        Value::True
    }

    fn unknow(expr: &mut Vec<&Token>, pos: usize) -> Value {
        Value::Unknow
    }

    fn bracket(expr: &mut Vec<&Token>, pos: usize) -> Value {
        expr.remove(pos);
        for i in pos..expr.len() {
            if expr[i].operator_type == Type::Bracket_open {
                bracket(expr, i);
            }
            if expr[i].operator_type == Type::Bracket_close {
                let res = super::super::eval(&mut expr[pos..i].to_vec());
                while expr[pos].operator_type != Type::Bracket_close {
                    expr.remove(pos);
                }
                expr.remove(pos);
                expr.insert(pos, if res { &True } else { &False });
            }
        }
        Unknow::True
    }

    fn negate(expr: &mut Vec<&Token>, pos: usize) -> Value {
        let res = (expr[pos + 1].exec)(expr, pos + 1);

        expr.remove(pos + 1);
        expr.remove(pos);

        match res {
            Value::True => expr.insert(pos, &True),//TODO function + finir bool=>Value
            Value::False => expr.insert(pos, &False),
            _ => expr.insert(pos, &Unknow),
        }

        res
    }

    fn and(expr: &mut Vec<&Token>, pos: usize) -> Value {
        let mut res = 0;
        if (expr[pos - 1].exec)(expr, pos - 1) && (expr[pos + 1].exec)(expr, pos + 1) {
            res = 1;
        }
        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        if res { expr.insert(pos - 1, &True); }
        else { expr.insert(pos - 1, &False); }
        res
    }

    fn or(expr: &mut Vec<&Token>, pos: usize) -> Value {
        let mut res = 0;
        if (expr[pos - 1].exec)(expr, pos - 1) || (expr[pos + 1].exec)(expr, pos + 1) {
            res = 1;
        }
        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        if res { expr.insert(pos - 1, &True); }
        else { expr.insert(pos - 1, &False); }
        res
    }

    fn xor(expr: &mut Vec<&Token>, pos: usize) -> i32 {
        let mut res = 0;
        let a = (expr[pos - 1].exec)(expr, pos - 1) as bool;//TODO unknow
        let b = (expr[pos + 1].exec)(expr, pos + 1) as bool;

        if (a || b) && (!a || !b) {
            res = 1;
        }
        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        if res { expr.insert(pos - 1, &True); }
        else { expr.insert(pos - 1, &False); }
        res
    }

    fn _false(expr: &mut Vec<&Token>, pos: usize) -> Value {
        Value::False
    }

    fn _true(expr: &mut Vec<&Token>, pos: usize) -> Value {
        Value::True
    }
}
