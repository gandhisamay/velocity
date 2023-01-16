#[derive(Debug)]
pub enum VeloError {
    ConnectionError(String),
}

impl std::fmt::Display for VeloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VeloError::ConnectionError(msg) => write!(f, "{}", msg),
        }
    }
    // add code here
}
