mod lightmappedgeneric;
mod unlitgeneric;
mod water;

pub use crate::material::unlitgeneric::UnlitGenericMaterial;
pub use lightmappedgeneric::LightMappedGenericMaterial;
use serde::{Deserialize, Serialize};
pub use water::WaterMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
    #[serde(rename = "lightmappedgeneric")]
    LightMappedGeneric(LightMappedGenericMaterial),
    #[serde(rename = "unlitgeneric")]
    UnlitGeneric(UnlitGenericMaterial),
    #[serde(rename = "water")]
    Water(WaterMaterial),
}
