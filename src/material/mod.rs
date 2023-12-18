mod lightmappedgeneric;
mod unlitgeneric;
mod water;
mod worldvertextransition;

pub use lightmappedgeneric::LightMappedGenericMaterial;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use unlitgeneric::UnlitGenericMaterial;
pub use water::WaterMaterial;
pub use worldvertextransition::WorldVertexTransitionMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
    #[serde(rename = "lightmappedgeneric")]
    LightMappedGeneric(LightMappedGenericMaterial),
    #[serde(rename = "unlitgeneric")]
    UnlitGeneric(UnlitGenericMaterial),
    #[serde(rename = "water")]
    Water(WaterMaterial),
    #[serde(rename = "worldvertextransition")]
    WorldVertexTransition(WorldVertexTransitionMaterial),
    #[serde(rename = "patch")]
    Patch(PatchMaterial),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMaterial {
    include: String,
    replace: HashMap<String, String>,
}
