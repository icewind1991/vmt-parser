use super::deserialize_path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulateMaterial {
    /// Multiplies the color behind it with this surface's texture.
    #[serde(rename = "$basetexture", deserialize_with = "deserialize_path")]
    pub base_texture: String,
    /// Doubles the modulation, making it appear brighter. 1 enables this, 0 disables. Disabled by default.
    #[serde(rename = "$mod2x", default)]
    pub mod_2x: bool,
    /// Disables backface culling.
    #[serde(rename = "$nocull", default)]
    pub no_cull: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SpriteOrientation {
    ParallelUpright,
    #[default]
    VpParallel,
    Oriented,
    VpParallelOriented,
}
