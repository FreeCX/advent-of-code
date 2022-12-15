fn has_sum(data: &[i64], item: i64) -> bool {
    let mut count = 0;
    for a in data {
        count += 1;
        for b in data.iter().skip(count) {
            if a + b == item {
                return true;
            }
        }
    }
    false
}

pub fn process(n: &[i64], header: usize) -> i64 {
    let mut pointer = header;
    for item in n.iter().skip(header) {
        let window = &n[pointer - header..pointer];
        if !has_sum(window, *item) {
            return *item;
        }
        pointer += 1;
    }
    unreachable!()
}
