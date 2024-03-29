#[cfg(test)]
mod tests;

fn increment(mut data: Vec<u8>) -> Vec<u8> {
    let (start, stop) = (b'a', b'z');
    let range = stop - start + 1;
    let mut carry = 1;
    for c in data.iter_mut().rev() {
        let next = *c - start + carry;
        *c = next % range + start;
        carry = u8::from(next == range);
    }
    data
}

// rule 1: passwords must include one increasing straight of at least three letters, like abc, bcd, cde,
// and so on, up to xyz. They cannot skip letters; abd doesn't count.
fn rule_one(data: &[u8]) -> bool {
    let mut seq_size = 1;
    let mut last = 0;
    for current in data {
        if last + 1 == *current {
            seq_size += 1;
            if seq_size >= 3 {
                return true;
            }
        } else {
            seq_size = 1;
        }
        last = *current;
    }
    seq_size >= 3
}

// rule 2: passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters
// and are therefore confusing.
fn rule_two(data: &[u8]) -> bool {
    let bad = "iol";
    for symbol in bad.bytes() {
        if data.contains(&symbol) {
            return false;
        }
    }
    true
}

// rule 3: passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz
fn rule_three(data: &[u8]) -> bool {
    let mut pairs_count = 0;
    let mut data_str: String = data.iter().map(|&x| x as char).collect();
    let pairs: Vec<String> = (b'a'..=b'z').map(|x| format!("{0}{0}", x as char)).collect();
    for pair in pairs {
        if data_str.contains(&pair) {
            pairs_count += 1;
            // remove this pair from text
            let index = data_str.find(&pair).unwrap();
            data_str = format!("{}##{}", &data_str[..index], &data_str[index + 2..]);
        }
    }
    pairs_count >= 2
}

fn next_password(input: &str) -> String {
    let mut repr: Vec<_> = input.bytes().collect();
    repr = increment(repr);
    while !(rule_one(&repr) && rule_two(&repr) && rule_three(&repr)) {
        repr = increment(repr);
    }
    String::from_utf8(repr).unwrap()
}

fn main() {
    let input = "vzbxkghb";
    let pass_one = next_password(input);
    let pass_two = next_password(&pass_one);
    println!("curr: {}\n one: {}\n two: {}", input, pass_one, pass_two);
}
