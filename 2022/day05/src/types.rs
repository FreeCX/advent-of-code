pub type Crate = char;
pub type Stack = Vec<Crate>;
pub type App = Vec<Command>;
pub type Ship = Vec<Stack>;

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    pub count: u16,
    pub from: u16,
    pub to: u16,
}

impl Command {
    pub fn new(count: u16, from: u16, to: u16) -> Self {
        Command { count, from, to }
    }
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let items: Vec<_> = value
            .split_whitespace()
            .filter(|&x| x.chars().next().unwrap().is_ascii_digit())
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        Command::new(items[0], items[1], items[2])
    }
}
