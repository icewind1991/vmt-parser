mod error;
pub mod material;

pub use error::VmtError;

pub type Result<T, E = VmtError> = std::result::Result<T, E>;
