use super::deserialize_path;
use crate::{
    default_detail_scale, default_scale, default_scale3, BlendMode, TextureTransform, Vec2, Vec3,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexLitGenericMaterial {
    /// Defines an albedo texture.
    #[serde(rename = "$basetexture", deserialize_with = "deserialize_path")]
    pub base_texture: String,
    /// Detail texturing.
    #[serde(rename = "$detail", default, deserialize_with = "deserialize_path")]
    pub detail: Option<String>,
    /// Use a 2nd UV channel for high-resolution decal support.
    #[serde(
        rename = "$decaltexture",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub decal_texture: Option<String>,

    /// Color tinting
    #[serde(rename = "$color2", default = "default_scale3")]
    pub color2: Vec3,
    /// Transforms the texture before use in the material. This does not affect lightmaps on the surface.
    #[serde(rename = "$basetexturetransform", default)]
    pub base_texture_transform: TextureTransform,
    /// Independently scales the red, green and blue channels of an albedo.
    #[serde(rename = "$color", default = "default_scale3")]
    pub color: Vec3,
    #[serde(rename = "$decalscale", default = "default_detail_scale")]
    /// Fits the detail texture onto the material the given number of times
    pub detail_scale: Vec2,
    /// Controls the amount that the detail texture affects the base texture. The precise use of this depends on the blend factor; in most cases it acts similarly to $alpha. A value of 0 usually makes the detail texture have no effect, whilst a value of 1 applies the full effect.
    #[serde(rename = "$detailblendfactor", default = "default_scale")]
    pub detail_blend_factor: f32,
    /// How to combine the detail material with the albedo.
    #[serde(rename = "$detailblendmode", default)]
    pub detail_blend_mode: BlendMode,
    /// A separate VertexLitGeneric material to that will replace this one if the decal hits a model.
    #[serde(
        rename = "$modelmaterial",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub model_material: Option<String>,
    /// Disables texture filtering.
    #[serde(rename = "$pointsamplemagfilter", default)]
    pub point_sample_mag_filter: bool,
    /// Mitigation for displacement texture stretching.
    #[serde(rename = "$seamless_scale", default = "default_scale")]
    pub seamless_scale: f32,

    /// Scales the opacity of an entire material.
    #[serde(rename = "$alpha", default = "default_scale")]
    pub alpha: f32,
    /// Specifies a mask to use to determine binary opacity.
    #[serde(rename = "$alphatest", default)]
    pub alpha_test: bool,
    /// Specifies a mask to use to determine binary opacity.
    #[serde(rename = "$alphatestreference", default = "default_scale")]
    pub alpha_test_reference: f32,
    /// Vector-like edge filtering.
    #[serde(rename = "$distancealpha", default)]
    pub distance_alpha: bool,
    /// Disables backface culling.
    #[serde(rename = "$nocull", default)]
    pub no_cull: bool,
    /// Specifies that the material should be partially see-through.
    #[serde(rename = "$translucent", default)]
    pub translucent: bool,

    /// Specifies a texture that will provide three-dimensional lighting information for a material.
    #[serde(rename = "$bumpmap", default, deserialize_with = "deserialize_path")]
    pub bump_map: Option<String>,
    /// Per-texel color modification via a warp texture.
    #[serde(
        rename = "$lightwarptexture",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub light_wrap_texture: Option<String>,
    /// Determines whether the surface is self-illuminated independent of environment lighting.
    #[serde(rename = "$selfillum", default)]
    pub self_illum: bool,
    /// Flags the $bumpmap as being a self-shadowing bumpmap.
    #[serde(rename = "$ssbump", default)]
    pub ss_bump: bool,

    /// Specular reflections.
    #[serde(rename = "$envmap", default, deserialize_with = "deserialize_path")]
    pub env_map: Option<String>,
    /// Diffuse reflections.
    #[serde(rename = "$phong", default)]
    pub phong: f32,

    /// Prevents fog from overdrawing a material.
    #[serde(rename = "$nofog", default)]
    pub no_fog: bool,

    /// Ignore z filtering
    #[serde(rename = "$ignorez", default)]
    pub ignore_z: bool,
}
