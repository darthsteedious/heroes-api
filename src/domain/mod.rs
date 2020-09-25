pub mod hero;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type ResultOption<T> = Result<Option<T>>;