use super::deserialize_path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableMaterial {
    /// Defines an albedo texture.
    #[serde(rename = "$basetexture", deserialize_with = "deserialize_path")]
    pub base_texture: String,
    /// Specifies a texture that will provide three-dimensional lighting information for a material.
    #[serde(rename = "$bumpmap", default, deserialize_with = "deserialize_path")]
    pub bump_map: Option<String>,

    /// Use computed vertex colors.
    #[serde(rename = "$vertexcolor", default)]
    pub vertex_color: bool,

    /// Minimum amount of light received
    #[serde(rename = "$minlight", default = "default_min_light")]
    pub min_light: f32,
    /// Maximum amount of light received
    #[serde(rename = "$maxlight", default = "default_max_light")]
    pub max_light: f32,

    /// Disables backface culling.
    #[serde(rename = "$nocull", default)]
    pub no_cull: bool,
}

fn default_min_light() -> f32 {
    0.1
}
fn default_max_light() -> f32 {
    0.3
}
