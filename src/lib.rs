mod data;
pub mod material;

use crate::material::Material;
pub use data::*;

pub type Result<T, E = VdfError> = std::result::Result<T, E>;

pub fn from_str(input: &str) -> Result<Material> {
    let input = input.to_ascii_lowercase();
    vdf_reader::from_str(&input)
}

pub use vdf_reader::VdfError;
