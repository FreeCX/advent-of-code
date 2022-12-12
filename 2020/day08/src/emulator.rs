use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum Opcode {
    Nop(i16),
    Acc(i16),
    Jmp(i16),
}

type Code = Vec<Opcode>;

pub struct Emulator {
    pub acc: i16,
    pub ip: i16,
    code: Code,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            acc: 0,
            ip: 0,
            code: Vec::new(),
        }
    }

    pub fn load(&mut self, code: &str) {
        for line in code.lines() {
            let items: Vec<_> = line.split(' ').collect();
            self.code.push(match &items[..] {
                &["nop", value] => Opcode::Nop(value.parse::<i16>().unwrap()),
                &["acc", value] => Opcode::Acc(value.parse::<i16>().unwrap()),
                &["jmp", value] => Opcode::Jmp(value.parse::<i16>().unwrap()),
                value => panic!("{:?} is unsupported", value),
            });
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }

    pub fn step(&mut self) {
        match self.code[self.ip as usize] {
            Opcode::Nop(_) => {}
            Opcode::Acc(value) => self.acc += value,
            Opcode::Jmp(value) => {
                self.ip += value;
                return;
            }
        }
        self.ip += 1;
    }

    pub fn has_loop(&mut self) -> bool {
        let mut ip = HashSet::<i16>::new();
        while (self.ip as usize) < self.code.len() {
            self.step();
            if ip.contains(&self.ip) {
                return true;
            }
            ip.insert(self.ip);
        }
        false
    }

    pub fn patch(&mut self, pos: i16) {
        let pos = pos as usize;
        self.code[pos] = match self.code[pos] {
            Opcode::Nop(value) => Opcode::Jmp(value),
            Opcode::Jmp(value) => Opcode::Nop(value),
            _ => unreachable!(),
        }
    }

    pub fn opcode(&self) -> Opcode {
        self.code[self.ip as usize]
    }
}

impl Opcode {
    pub fn is_nop(&self) -> bool {
        matches!(self, Opcode::Nop(_))
    }

    pub fn is_jmp(&self) -> bool {
        matches!(self, Opcode::Jmp(_))
    }
}
