#![allow(dead_code, unused_variables)]
use std::collections::HashSet;

enum Opcode {
    Nop,
    Acc(i16),
    Jmp(i16),
}

type Code = Vec<Opcode>;

struct Emulator {
    acc: i32,
    ip: i32,
    code: Code,
}

impl Emulator {
    fn new() -> Emulator {
        Emulator {
            acc: 0,
            ip: 0,
            code: Vec::new(),
            visited: HashSet::new(),
        }
    }

    fn load(&mut self, code: &str) {
        for line in code.lines() {
            let items: Vec<_> = line.split(' ').collect();
            self.code.push(match &items[..] {
                &["nop", value] => Opcode::Nop,
                &["acc", value] => Opcode::Acc(value.parse::<i16>().unwrap()),
                &["jmp", value] => Opcode::Jmp(value.parse::<i16>().unwrap()),
            });
        }
    }

    fn execute(&mut self) -> i32 {
        todo!()
    }
}

fn main() {}
