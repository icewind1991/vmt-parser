mod error;
pub mod vdf;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
