use super::deserialize_path;
use crate::TextureTransform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyMaterial {
    /// Defines an albedo texture.
    #[serde(rename = "$basetexture", deserialize_with = "deserialize_path")]
    pub base_texture: String,
    /// Defines an albedo texture.
    #[serde(
        rename = "$hdrbasetexture",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub hdr_base_texture: Option<String>,
    /// Links the surface to a set of physical properties.
    #[serde(rename = "$surfaceprop", default)]
    pub surface_prop: Option<String>,
    /// Transforms the texture before use in the material. This does not affect lightmaps on the surface.
    #[serde(rename = "$basetexturetransform", default)]
    pub base_texture_transform: TextureTransform,

    /// Ignore z filtering
    #[serde(rename = "$ignorez", default)]
    pub ignore_z: bool,

    /// Prevents fog from overdrawing a material.
    #[serde(rename = "$nofog", default)]
    pub no_fog: bool,

    #[serde(rename = "$nomip", default)]
    pub no_mip: bool,
}
