use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

struct R {
    left: Pos,
    right: Pos,
    now: Pos,
    start: bool,
}

enum Command {
    TurnOn(Pos, Pos),
    Toggle(Pos, Pos),
    TurnOff(Pos, Pos),
}

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw: Vec<_> = s.split(',').collect();
        let x = raw[0].parse()?;
        let y = raw[1].parse()?;
        Ok(Pos { x, y })
    }
}

impl Iterator for R {
    type Item = Pos;

    // TODO: отрефакторить код
    fn next(&mut self) -> Option<Self::Item> {
        if !self.start {
            if self.now.x >= self.right.x {
                self.now.x = self.left.x;
                if self.now.y >= self.right.y {
                    return None;
                } else {
                    self.now.y += 1;
                }
            } else {
                self.now.x += 1;
            }
        } else {
            self.start = false;
        }
        Some(self.now)
    }
}

impl R {
    fn new(a: Pos, b: Pos) -> R {
        R { left: a, right: b, now: a, start: true }
    }
}

fn parse(cmd: &str) -> Command {
    let raw: Vec<_> = cmd.split(' ').collect();
    match raw[..] {
        ["turn", "on", a, "through", b] => Command::TurnOn(a.parse().unwrap(), b.parse().unwrap()),
        ["turn", "off", a, "through", b] => Command::TurnOff(a.parse().unwrap(), b.parse().unwrap()),
        ["toggle", a, "through", b] => Command::Toggle(a.parse().unwrap(), b.parse().unwrap()),
        _ => unreachable!(),
    }
}

fn main() {
    let mut grid = [[(false, 0); 1000]; 1000];
    let buffer = include_str!("../data/input");

    for item in buffer.lines() {
        match parse(item) {
            Command::TurnOn(a, b) => {
                for p in R::new(a, b) {
                    grid[p.x][p.y].0 = true;
                    grid[p.x][p.y].1 += 1;
                }
            }
            Command::TurnOff(a, b) => {
                for p in R::new(a, b) {
                    grid[p.x][p.y].0 = false;
                    if grid[p.x][p.y].1 > 1 {
                        grid[p.x][p.y].1 -= 1;
                    } else {
                        grid[p.x][p.y].1 = 0;
                    }
                }
            }
            Command::Toggle(a, b) => {
                for p in R::new(a, b) {
                    grid[p.x][p.y].0 = !(grid[p.x][p.y].0);
                    grid[p.x][p.y].1 += 2;
                }
            }
        }
    }

    let mut count = 0;
    let mut total_brightness = 0;
    for row in grid {
        for item in row {
            if item.0 {
                count += 1;
            }
            total_brightness += item.1;
        }
    }

    println!("count = {}", count);
    println!("total brightness = {}", total_brightness);
}
