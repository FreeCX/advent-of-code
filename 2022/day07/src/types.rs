#[derive(Debug)]
pub enum LineType {
    ChangeDirectory(String),
    ExitDirectory,
    List,
    Directory(String),
    File { size: u32, name: String },
}

impl From<&str> for LineType {
    fn from(value: &str) -> Self {
        match &value.split_whitespace().collect::<Vec<_>>()[..] {
            &["$", "cd", ".."] => LineType::ExitDirectory,
            &["$", "cd", name] => LineType::ChangeDirectory(name.to_string()),
            &["$", "ls"] => LineType::List,
            &["dir", name] => LineType::Directory(name.to_string()),
            &[size, name] => LineType::File { size: size.parse().unwrap(), name: name.to_string() },
            other => panic!("unsupported format: {:?}", other),
        }
    }
}
