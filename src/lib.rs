mod data;
mod error;
pub mod material;

use crate::material::Material;
pub use data::*;
pub use error::VmtError;

pub type Result<T, E = VmtError> = std::result::Result<T, E>;

pub fn from_str(input: &str) -> Result<Material> {
    let input = input.to_ascii_lowercase();
    vdf_reader::from_str(&input).map_err(VmtError::from)
}
