pub fn priority(symbol: char) -> u32 {
    match symbol {
        'a'..='z' => symbol as u32 - b'a' as u32 + 1,
        'A'..='Z' => symbol as u32 - b'A' as u32 + 27,
        symbol => panic!("symbol `{}` in unsupported", symbol),
    }
}
