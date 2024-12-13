#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Status {
    Done,
    #[default]
    Incomplete,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            Status::Done => "x".to_string(),
            Status::Incomplete => " ".to_string(),
        };
        write!(f, "{string}")
    }
}

impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "x" => Ok(Status::Done),
            " " => Ok(Status::Incomplete),
            c => Err(format!("Invalid Status character: {c}")),
        }
    }
}
