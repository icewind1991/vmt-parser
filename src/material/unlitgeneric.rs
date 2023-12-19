use super::deserialize_path;
use crate::{default_scale, default_scale3, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlitGenericMaterial {
    /// Defines an albedo texture.
    #[serde(rename = "$basetexture", deserialize_with = "deserialize_path")]
    pub base_texture: String,
    /// Links the surface to a set of physical properties.
    #[serde(rename = "$surfaceprop", default)]
    pub surface_prop: Option<String>,
    /// Tells this material is used for models and not brushes.
    #[serde(rename = "$model", default)]
    pub model: bool,

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
    /// Disables backface culling.
    #[serde(rename = "$nocull", default)]
    pub no_cull: bool,
    /// Specifies that the material should be partially see-through.
    #[serde(rename = "$translucent", default)]
    pub translucent: bool,

    /// Specifies a texture that will provide three-dimensional lighting information for a material.
    #[serde(rename = "$bumpmap", default, deserialize_with = "deserialize_path")]
    pub bump_map: Option<String>,

    /// Ignore z filtering
    #[serde(rename = "$ignorez", default)]
    pub ignore_z: bool,

    /// Prevents fog from overdrawing a material.
    #[serde(rename = "$nofog", default)]
    pub no_fog: bool,

    /// Allow the player's flashlight to illuminate the material.
    #[serde(rename = "$receiveflashlight", default)]
    pub receive_flash_light: bool,
    /// Possibly used to allow shadows to form from this decal.
    #[serde(rename = "singlepassflashlight", default)]
    pub single_pass_flash_light: bool,
    #[serde(rename = "$no_fullbright", default)]
    pub no_full_bright: bool,
}
