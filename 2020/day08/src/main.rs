use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Opcode {
    Nop(i16),
    Acc(i16),
    Jmp(i16),
}

type Code = Vec<Opcode>;

struct Emulator {
    pub acc: i16,
    pub ip: i16,
    code: Code,
}

impl Emulator {
    fn new() -> Emulator {
        Emulator { acc: 0, ip: 0, code: Vec::new() }
    }

    fn load(&mut self, code: &str) {
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

    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }

    fn step(&mut self) {
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

    fn has_loop(&mut self) -> bool {
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

    fn patch(&mut self, pos: i16) {
        let pos = pos as usize;
        self.code[pos] = match self.code[pos] {
            Opcode::Nop(value) => Opcode::Jmp(value),
            Opcode::Jmp(value) => Opcode::Nop(value),
            _ => unreachable!(),
        }
    }

    fn opcode(&self) -> Opcode {
        self.code[self.ip as usize]
    }
}

impl Opcode {
    fn is_nop(&self) -> bool {
        match self {
            Opcode::Nop(_) => true,
            _ => false,
        }
    }

    fn is_jmp(&self) -> bool {
        match self {
            Opcode::Jmp(_) => true,
            _ => false,
        }
    }
}

fn task_one(code: &str) -> i16 {
    let mut ip = HashSet::<i16>::new();
    let mut emulator = Emulator::new();
    emulator.load(code);

    loop {
        emulator.step();
        if ip.contains(&emulator.ip) {
            return emulator.acc;
        }
        ip.insert(emulator.ip);
    }

    unreachable!()
}

fn task_two(code: &str) -> i16 {
    let mut ip = HashSet::<i16>::new();
    let mut emulator = Emulator::new();
    emulator.load(code);

    // collect nops & jmps
    let mut indexes = Vec::new();
    'main: loop {
        let opcode = emulator.opcode();
        if opcode.is_nop() || opcode.is_jmp() {
            indexes.push(emulator.ip);
        }
        emulator.step();
        if ip.contains(&emulator.ip) {
            break 'main;
        }
        ip.insert(emulator.ip);
    }

    // patch & check
    for index in indexes.iter().rev() {
        emulator.reset();
        emulator.patch(*index);
        if !emulator.has_loop() {
            return emulator.acc;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use crate::*;

    const APP: &'static str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

    #[test]
    fn example_one() {
        assert_eq!(task_one(APP), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(task_two(APP), 8);
    }
}

fn main() {
    let app = include_str!("../input");

    println!(" first = {}", task_one(app));
    println!("second = {}", task_two(app));
}
