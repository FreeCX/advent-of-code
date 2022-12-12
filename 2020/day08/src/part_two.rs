use std::collections::HashSet;
use crate::emulator;

pub fn process(code: &str) -> i16 {
    let mut ip = HashSet::<i16>::new();
    let mut emulator = emulator::Emulator::new();
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
