#[derive(Debug)]
pub enum VeloError {
    FileIOError(String),
    BookmarkOpenFailure(String),
}

impl std::fmt::Display for VeloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VeloError::FileIOError(msg) => write!(f, "{}", msg),
            VeloError::BookmarkOpenFailure(msg) => write!(f, "{}", msg),
        }
    }
    // add code here
}
