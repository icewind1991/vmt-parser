mod lightmappedgeneric;
mod unlitgeneric;

pub use crate::material::unlitgeneric::UnlitGenericMaterial;
pub use lightmappedgeneric::LightMappedGenericMaterial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
    #[serde(rename = "lightmappedgeneric")]
    LightMappedGeneric(LightMappedGenericMaterial),
    #[serde(rename = "unlitgeneric")]
    UnlitGeneric(UnlitGenericMaterial),
}
