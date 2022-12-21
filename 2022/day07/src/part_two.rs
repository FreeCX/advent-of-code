use crate::part_one;
use crate::types::LineType;

pub fn process(input: &[LineType]) -> u32 {
    let total_fs_size = 70_000_000;
    let need_at_least = 30_000_000;

    let report = part_one::report(input);

    let used_space = *report.get("//").unwrap();
    let need_space = need_at_least - (total_fs_size - used_space);

    let mut sizes: Vec<_> = report.into_values().collect();
    sizes.sort();

    for size in sizes.into_iter() {
        if size >= need_space {
            return size;
        }
    }

    unreachable!()
}
