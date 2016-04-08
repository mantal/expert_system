extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rule;
use rule::Rule;
use token;
use token::Token;
use token::Operators;

impl String {
    fn to_graphemes(&self) -> Vec<&str> {
        UnicodeSegmentation::graphemes(self, true).collect::<Vec<&str>>()
    }
}

pub fn get_file(path: String) -> String {
    let real_path = Path::new(&path); 
    let mut file = match File::open(&real_path) {
        Err(e) => panic!("Could not open {}: {}", real_path.display(), Error::description(&e)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("Could not read {}: {}", real_path.display(), Error::description(&e)),
        Ok(_) => s,
    }
}

fn charset(c: char) -> bool{
    c.is_alphabetic() || "!+*|@^~()=i⊤⊥⇒⇔¬·⊕⊼∥⊽".contains(c) || c.is_whitespace()
}

pub fn facts(mut line: String, rules: &mut Vec<Rule>) {
    line.remove(0);
    if line.chars().filter(|&e| !e.is_alphabetic()).count() > 0 {
        panic!("Syntax error: facts declaration can only contain variable");
    }
    for c in line.chars() {
        rules.push(Rule { variable: c, rule: [Operators::True].to_vec() });
    }
}

pub fn rule(mut line: String, rules: &mut Vec<Rule>) {
    expr_to_rule(rules, &line.chars().map(|c| match c {
        '!' | '¬' => Operators::Negate,
        '+' | '·' => Operators::And,
        '*' | '⊼' => Operators::Nand,
        '|' | '∥' => Operators::Or,
        '@' | '⊽' => Operators::Nor,
        '^' | '⊕' => Operators::Xor,
        '~' => Operators::Xnor,
        '(' => Operators::Bracket_open,
        ')' => Operators::Bracket_close,
        '⇒' => Operators::Implies,
        '⇔' => Operators::Equivalent,
        _ => Operators::new_variable(c),
    }).collect::<Vec<_>>());
}

pub fn query(mut line: String, rules: &Vec<Rule>) {
    line.remove(0);
    
    if line.chars().filter(|&e| !e.is_alphabetic()).count() > 0 {
        panic!("Syntax error: querys can only contain variable");
    }
    
    for c in line.chars() {
        println!("{}: {:?}", c, rule::query(rules.clone(), c));
    }
}

pub fn parse_file(file: String, rules: &mut Vec<Rule>) {
    for line in file.lines() {
        let expr = line.replace("=>", "⇒").replace("<=>", "⇔").chars().take_while(|&e| e != '#').collect::<String>();
        if line.chars().filter(|&e| !charset(e)).count() > 0 {
            panic!("Syntax error: unknow token");
        }
        match expr.chars().next().unwrap() {
            '#' => continue,
            '=' => facts(line.to_string(), rules),
            '?' => query(line.to_string(), rules),
            _   => rule(line.to_string(), rules),
        }
    }
}

pub fn expr_to_rule(rules:  &mut Vec<Rule>, expr: &Vec<Token>) {
    let i = match expr.iter().position(|e| e.operator_type == Operators::Type::implies
                                       || e.operator_type == Operators::Type::if_and_only_if) {
        Some(e) => e,
        None => panic!("Syntax error: rule has no right side"),
    };
    let mut left = expr.clone();
    let mut right = left.split_off(i);
    right.remove(0);
    match right.len() {
        0 => panic!("Syntax error: right side has no operand"),
        1 => {
            match right[0].operator_type {
                Operators::Type::Operand{name} => (),
                _ => panic!("Syntax error: right side has no operand"),
            }
            rules.push(Rule { variable: right[0].get_name(), rule: left.clone() });
        },
        2 => {
            left.insert(0, Operators::Bracket_open);
            left.push(Operators::Bracket_close);
            left.insert(0, Operators::Negate);
            rules.push(Rule { variable: right[0].get_name(), rule: left.clone() });
        }
        3 => {
            match right[0].operator_type {
                Operators::Type::Operand{name} => (),
                _ => panic!("Syntax error"),
            }
            match right[1].priority  {//right[1] == Token::And TODO better
                2200 => (),
                _ => panic!("Unsupported operation or syntax error"),//TODO better
            }
            match right[2].operator_type {
                Operators::Type::Operand{name} => (),
                _ => panic!("Syntax error"),
            }
            rules.push(Rule { variable: right[0].get_name(), rule: left.clone() });
            rules.push(Rule { variable: right[2].get_name(), rule: left.clone() });
        }
        _ => panic!("Unsupported operation"),
    }
}
