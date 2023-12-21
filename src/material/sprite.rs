use super::deserialize_path;
use crate::{default_scale, default_scale3, Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteMaterial {
    /// Defines an albedo texture.
    #[serde(rename = "$basetexture", deserialize_with = "deserialize_path")]
    pub base_texture: String,
    /// Links the surface to a set of physical properties.
    #[serde(rename = "$surfaceprop", default)]
    pub surface_prop: Option<String>,

    #[serde(rename = "$spriteorientation", default)]
    pub sprite_orientation: SpriteOrientation,
    #[serde(rename = "$spriteorigin", default)]
    pub sprite_origin: Vec2,

    /// Independently scales the red, green and blue channels of an albedo.
    #[serde(rename = "$color", default = "default_scale3")]
    pub color: Vec3,

    /// Scales the opacity of an entire material.
    #[serde(rename = "$alpha", default = "default_scale")]
    pub alpha: f32,
    /// Specifies a mask to use to determine binary opacity.
    #[serde(rename = "$alphatest", default)]
    pub alpha_test: bool,
    /// Specifies a mask to use to determine binary opacity.
    #[serde(rename = "$alphatestreference", default = "default_scale")]
    pub alpha_test_reference: f32,
    /// Specifies that the material should be partially see-through.
    #[serde(rename = "$translucent", default)]
    pub translucent: bool,
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
