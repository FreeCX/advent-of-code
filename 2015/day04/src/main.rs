extern crate md5;
use md5::compute;

fn main() {
    let mut first_five_not_found = true;
    let key = "yzbqklnj";

    for i in 0..std::u64::MAX {
        let output = compute(format!("{}{}", key, i).as_bytes());
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        let first_six = output[0] as i32 + output[1] as i32 + output[2] as i32;

        if first_five_not_found && first_five == 0 {
            println!("first five: {}", i);
            first_five_not_found = false;
        }

        if first_six == 0 {
            println!("first six: {}", i);
            break;
        }
    }
}
