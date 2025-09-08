// create enum to handler error


#[derive(Debug)]
pub enum CustomError {
    ParseError(String),
    TransformationError(String),
    IOError(String),
    UnknownError(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            CustomError::TransformationError(msg) => write!(f, "Transformation Error: {}", msg),
            CustomError::IOError(msg) => write!(f, "IO Error: {}", msg),
            CustomError::UnknownError(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}