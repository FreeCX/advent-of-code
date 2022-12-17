use crate::types::{App, Command, Ship, Stack};

fn parse_stacks(input: &str) -> Ship {
    // iterate in reverse order, skipping the first line
    //     [D]     | 4
    // [N] [C]     | 3
    // [Z] [M] [P] | 2
    //  1   2   3  | 1  <- skip
    let mut iterator = input.lines().rev().skip(1).peekable();

    // calculate stack size by uppercase characters
    let size = if let Some(item) = iterator.peek() {
        item.chars().filter(|&x| x.is_uppercase()).count()
    } else {
        panic!("no data")
    };

    let mut ship = vec![Stack::new(); size];

    for line in iterator {
        // select characters only from certain positions
        // [A] [B] [C]     [D] [E] [F]     [G]
        // -1---5---9---3---7---1---5---9---3-
        //  +   +   +   -   +   +   +   -   +
        let line_iter = line.chars().enumerate().filter(|&(x, _)| x % 4 == 1).map(|(_, y)| y);
        for (stack, item) in ship.iter_mut().zip(line_iter) {
            // ignore whitespace character
            if !item.is_whitespace() {
                stack.push(item);
            }
        }
    }

    ship
}

fn parse_commands(input: &str) -> App {
    let mut commands = Vec::new();
    for line in input.trim().lines() {
        commands.push(Command::from(line));
    }
    commands
}

pub fn parse(input: &str) -> (Ship, App) {
    // INFO: use "\r\n\r\n" on windows encoded files
    let index = input.find("\n\n").unwrap();
    let (stacks, commands) = input.split_at(index);
    (parse_stacks(stacks), parse_commands(commands))
}
