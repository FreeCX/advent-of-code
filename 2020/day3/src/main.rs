type Pos = (usize, usize);

#[derive(Eq, PartialEq)]
enum Field {
    Empty,
    Tree,
}

struct Land {
    data: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

fn parse(data: &str) -> Land {
    let mut land = Vec::new();

    for line in data.lines() {
        let mut block = Vec::new();
        for item in line.chars() {
            block.push(match item {
                '.' => Field::Empty,
                '#' => Field::Tree,
                _ => panic!("unknown item"),
            });
        }
        land.push(block);
    }

    let width = land[0].len();
    let height = land.len();

    Land {
        data: land,
        width,
        height,
    }
}

fn task_one(slope: Pos, land: &Land) -> usize {
    let mut pos = slope;
    let mut counter = 0;

    while pos.1 < land.height {
        if land.data[pos.1][pos.0 % land.width] == Field::Tree {
            counter += 1;
        }
        pos.0 += slope.0;
        pos.1 += slope.1;
    }

    counter
}

fn task_two(land: &Land) -> usize {
    let r11 = task_one((1, 1), land);
    let r31 = task_one((3, 1), land);
    let r51 = task_one((5, 1), land);
    let r71 = task_one((7, 1), land);
    let r12 = task_one((1, 2), land);

    r11 * r31 * r51 * r71 * r12
}

fn main() {
    let land = parse(include_str!("../input"));

    println!(" first = {}", task_one((3, 1), &land));
    println!("second = {}", task_two(&land));
}
