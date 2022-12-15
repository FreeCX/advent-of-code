use std::collections::HashSet;

use crate::emulator;

pub fn process(code: &str) -> i16 {
    let mut ip = HashSet::<i16>::new();
    let mut emulator = emulator::Emulator::new();
    emulator.load(code);

    loop {
        emulator.step();
        if ip.contains(&emulator.ip) {
            return emulator.acc;
        }
        ip.insert(emulator.ip);
    }
}
