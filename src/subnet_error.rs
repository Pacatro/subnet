use std::error::Error;

#[derive(Debug)]
pub enum SubnetError {
    InvalidNumberHosts(String),
    ParserError(String),
}

impl std::fmt::Display for SubnetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubnetError::InvalidNumberHosts(msg) => write!(f, "{}", msg),
            SubnetError::ParserError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for SubnetError {}