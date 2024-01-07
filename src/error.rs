#[derive(Debug)]
pub enum Error{
    IO(std::io::Error),
    File(String)
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
           Self::IO(error) => {write!(f,"{error}")},
           Self::File(error) => {write!(f,"{error}")}
        }
    }
}

impl std::error::Error for Error{}

impl From<std::io::Error> for Error{
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}