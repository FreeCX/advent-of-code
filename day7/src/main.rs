use std::collections::BTreeMap;
use std::io::prelude::Read;
use std::fs::File;

#[derive(Debug, Clone)]
enum Operator {
    AND,
    OR,
    RSHIFT,
    LSHIFT,
    NOT,
    ERROR
}

#[derive(Debug, Clone)]
enum Token {
    // operator a b var
    Expression(Operator, String, String, String),
    // operator a var
    Inverter(String, String),
    // a var
    Statement(String, String),
    // result data
    Result(String, u16),
    // unknown syntax
    UnknownSyntax
}

fn get_op(op: &str) -> Operator {
    match op {
        "AND" => Operator::AND,
        "OR" => Operator::OR,
        "RSHIFT" => Operator::RSHIFT,
        "LSHIFT" => Operator::LSHIFT,
        "NOT" => Operator::NOT,
        _ => Operator::ERROR,
    }
}

fn parse(s: &str) -> Token {
    let data: Vec<_> = s.split_whitespace().map(|x| x.trim()).collect();
    match &data[..] {
        // a OP b -> v
        [a, op, b, "->", variable] => {
            Token::Expression(get_op(op), a.to_string(), b.to_string(), variable.to_string())
        },
        // NOT a -> v
        ["NOT", a, "->", variable] => {
            Token::Inverter(a.to_string(), variable.to_string())
        }
        // a -> v
        [number, "->", variable] => {
            Token::Statement(number.to_string(), variable.to_string())
        }
        _ => Token::UnknownSyntax,
    }
}

fn execute(expr: Token, vars: &BTreeMap<String, u16>) -> Option<Token> {
    match expr {
        Token::Statement(a, var) => {
            match a.parse() {
                Ok(numb) => Some(Token::Result(var, numb)),
                Err(_) => {
                    match vars.get(&a) {
                        Some(value) => {
                            Some(Token::Result(var, *value))
                        }
                        None => Some(Token::Statement(a, var))
                    }
                }
            }
        }
        Token::Expression(operator, a, b, var) => {
            let a_value = match vars.get(&a) {
                Some(value) => value.clone(),
                None => {
                    match a.parse() {
                        Ok(value) => value,
                        Err(_) => { return None; }
                    }
                },
            };
            match operator {
                Operator::AND => {
                    vars.get(&b).and_then(|b_value| {
                        Some(Token::Result(var, a_value & b_value))
                    })
                },
                Operator::OR => {
                    vars.get(&b).and_then(|b_value| {
                        Some(Token::Result(var, a_value | b_value))
                    })
                },
                Operator::LSHIFT => {
                    let b_value = b.parse::<u16>().unwrap();
                    Some(Token::Result(var, a_value << b_value))
                },
                Operator::RSHIFT => {
                    let b_value = b.parse::<u16>().unwrap();
                    Some(Token::Result(var, a_value >> b_value))
                },
                _ => None
            }
        },
        Token::Inverter(a, var) => {
            vars.get(&a).and_then(|a_value| Some(Token::Result(var, !a_value)))
        }
        _ => None
    }
}

fn main() {
    let mut vars: BTreeMap<String, u16> = BTreeMap::new();
    let mut stack: Vec<Token> = Vec::new();
    let mut all_expr: Vec<Token> = Vec::new();
    let mut f = File::open("input.txt").expect("[error] can't open file!");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("[error] can't read from file!");
    // подумать насчёт многопараметровой сортировки
    // 1. сортировка по выходному параметру
    // 2. сортировка по приоритету функции
    // -- Result > Statement > Inverter > Expression
    for expression in buffer.lines() {
        let expr = parse(expression);
        all_expr.push(expr.clone());
        match execute(expr.clone(), &vars) {
            Some(Token::Result(var, num)) => { vars.insert(var, num); },
            Some(Token::Statement(a, var)) => { stack.push(Token::Statement(a, var)); },
            _ => { stack.push(expr); },
        };
    }
    while !vars.get("a").is_some() {
        let new_stack = stack.clone();
        stack.clear();
        for expr in new_stack {
            match execute(expr.clone(), &vars) {
                Some(Token::Result(var, num)) => { vars.insert(var, num); },
                Some(Token::Statement(a, var)) => { stack.push(Token::Statement(a, var)); },
                _ => { stack.push(expr); },
            };
        }
    }
    let a = vars.get("a").unwrap();
    println!("a = {}", a);
}
