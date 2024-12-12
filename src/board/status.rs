#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Status {
    Done,
    #[default]
    Incomplete,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Done => "x".to_string(),
            Status::Incomplete => " ".to_string(),
        }
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
