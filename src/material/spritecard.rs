use super::deserialize_path;
use crate::{default_scale, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteCardMaterial {
    /// Defines an albedo texture.
    #[serde(
        rename = "$basetexture",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub base_texture: Option<String>,
    /// Links the surface to a set of physical properties.
    #[serde(rename = "$surfaceprop", default)]
    pub surface_prop: Option<String>,

    #[serde(rename = "$spriteorigin", default)]
    pub sprite_origin: Vec2,

    #[serde(rename = "$additive", default)]
    pub additive: bool,

    #[serde(rename = "$overbrightfactor", default)]
    pub over_bright_factor: f32,

    /// Use computed vertex colors.
    #[serde(rename = "$vertexcolor", default)]
    pub vertex_color: bool,
    /// Use computed vertex alpha.
    #[serde(rename = "$vertexalpha", default)]
    pub vertex_alpha: bool,

    /// Scales the opacity of an entire material.
    #[serde(rename = "$alpha", default = "default_scale")]
    pub alpha: f32,
    /// Specifies that the material should be partially see-through.
    #[serde(rename = "$translucent", default)]
    pub translucent: bool,
    /// Disables backface culling.
    #[serde(rename = "$nocull", default)]
    pub no_cull: bool,

    /// Multiply the output by 2x.
    #[serde(rename = "$mod2x", default)]
    pub mod_2x: bool,
    /// Are we opaque? Default 0.
    #[serde(rename = "$opaque", default)]
    pub opaque: bool,

    /// Multiply output RGB by intensity factor.
    #[serde(rename = "$intensity", default = "default_scale")]
    pub intensity: f32,
}
