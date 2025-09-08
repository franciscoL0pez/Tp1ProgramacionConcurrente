// create enum to handler error


#[derive(Debug)]
pub enum Error {
    ParseError(String),
    TransformationError(String),
    IOError(String),
    UnknownError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            Error::TransformationError(msg) => write!(f, "Transformation Error: {}", msg),
            Error::IOError(msg) => write!(f, "IO Error: {}", msg),
            Error::UnknownError(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}