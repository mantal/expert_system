pub struct Token {
    pub operator_type: Operators::Type,
    pub priority: i32,
    pub exec: fn(&mut Vec<&Token>, usize) -> bool
}

pub mod Operators {

    use token::Token;

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Type {
        Operand,
        Unary,
        Binary,
        Bracket_open,
        Bracket_close
    }

    pub static Negate: Token = Token { priority: 3000, exec: negate, operator_type: Type::Unary };
    pub static And: Token = Token { priority: 2200, exec: and, operator_type: Type::Binary };
    pub static Xor: Token = Token { priority: 2000, exec: xor, operator_type: Type::Binary };
    pub static Or: Token = Token { priority: 2100, exec: or, operator_type: Type::Binary };
    pub static True: Token = Token { priority: 0, exec: _true, operator_type: Type::Operand };
    pub static False: Token = Token { priority: 0, exec: _false, operator_type: Type::Operand };
    pub static Bracket_open: Token = Token { priority: 4000, exec: bracket, operator_type: Type::Bracket_open };
    pub static Bracket_close: Token = Token { priority: -1, exec: _false, operator_type: Type::Bracket_close };

    fn bracket(expr: &mut Vec<&Token>, pos: usize) -> bool {
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
        true
    }

    fn negate(expr: &mut Vec<&Token>, pos: usize) -> bool {
        let mut res = false;
        if !(expr[pos + 1].exec)(expr, pos + 1) {
            res = true;
        }
        expr.remove(pos + 1);
        expr.remove(pos);
        if res { expr.insert(pos, &True); }
        else { expr.insert(pos, &False); }
        res
    }

    fn and(expr: &mut Vec<&Token>, pos: usize) -> bool {
        let mut res = false;
        if (expr[pos - 1].exec)(expr, pos - 1) && (expr[pos + 1].exec)(expr, pos + 1) {
            res = true;
        }
        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        if res { expr.insert(pos - 1, &True); }
        else { expr.insert(pos - 1, &False); }
        res
    }

    fn or(expr: &mut Vec<&Token>, pos: usize) -> bool {
        let mut res = false;
        if (expr[pos - 1].exec)(expr, pos - 1) || (expr[pos + 1].exec)(expr, pos + 1) {
            res = true;
        }
        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        if res { expr.insert(pos - 1, &True); }
        else { expr.insert(pos - 1, &False); }
        res
    }

    fn xor(expr: &mut Vec<&Token>, pos: usize) -> bool {
        let mut res = false;
        let a = (expr[pos - 1].exec)(expr, pos - 1);
        let b = (expr[pos + 1].exec)(expr, pos + 1);

        if (a || b) && (!a || !b) {
            res = true;
        }
        expr.remove(pos + 1);
        expr.remove(pos);
        expr.remove(pos - 1);
        if res { expr.insert(pos - 1, &True); }
        else { expr.insert(pos - 1, &False); }
        res
    }

    fn _false(expr: &mut Vec<&Token>, pos: usize) -> bool {
        false
    }

    fn _true(expr: &mut Vec<&Token>, pos: usize) -> bool {
        true
    }
}
